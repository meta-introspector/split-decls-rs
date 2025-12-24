use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An atomic storage for a reference counted smart pointer like [`Arc`] or `Option<Arc>`.
///
/// This is a storage where a smart pointer may live. It can be read and written atomically from
/// several threads, but doesn't act like a pointer itself.
///
/// One can be created [`from`] an [`Arc`]. To get the pointer back, use the
/// [`load`](#method.load).
///
/// # Note
///
/// This is the common generic implementation. This allows sharing the same code for storing
/// both `Arc` and `Option<Arc>` (and possibly other similar types).
///
/// In your code, you most probably want to interact with it through the
/// [`ArcSwap`](type.ArcSwap.html) and [`ArcSwapOption`](type.ArcSwapOption.html) aliases. However,
/// the methods they share are described here and are applicable to both of them. That's why the
/// examples here use `ArcSwap` ‒ but they could as well be written with `ArcSwapOption` or
/// `ArcSwapAny`.
///
/// # Type parameters
///
/// * `T`: The smart pointer to be kept inside. This crate provides implementation for `Arc<_>` and
///   `Option<Arc<_>>` (`Rc` too, but that one is not practically useful). But third party could
///   provide implementations of the [`RefCnt`] trait and plug in others.
/// * `S`: Chooses the [strategy] used to protect the data inside. They come with various
///   performance trade offs, the default [`DefaultStrategy`] is good rule of thumb for most use
///   cases.
///
/// # Examples
///
/// ```rust
/// # use std::sync::Arc;
/// # use arc_swap::ArcSwap;
/// let arc = Arc::new(42);
/// let arc_swap = ArcSwap::from(arc);
/// assert_eq!(42, **arc_swap.load());
/// // It can be read multiple times
/// assert_eq!(42, **arc_swap.load());
///
/// // Put a new one in there
/// let new_arc = Arc::new(0);
/// assert_eq!(42, *arc_swap.swap(new_arc));
/// assert_eq!(0, **arc_swap.load());
/// ```
///
/// # Known bugs
///
/// Currently, things like `ArcSwapAny<Option<Option<Arc<_>>>>` (notice the double Option) don't
/// work properly. A proper solution is being looked into
/// ([#81](https://github.com/vorner/arc-swap/issues)).
///
/// [`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
/// [`from`]: https://doc.rust-lang.org/nightly/std/convert/trait.From.html#tymethod.from
/// [`RefCnt`]: trait.RefCnt.html
pub struct ArcSwapAny<T: RefCnt, S: Strategy<T> = DefaultStrategy> {
    /// The actual pointer, extracted from the Arc.
    ptr: AtomicPtr<T::Base>,
    /// We are basically an Arc in disguise. Inherit parameters from Arc by pretending to contain
    /// it.
    _phantom_arc: PhantomData<T>,
    /// Strategy to protect the data.
    strategy: S,
}
