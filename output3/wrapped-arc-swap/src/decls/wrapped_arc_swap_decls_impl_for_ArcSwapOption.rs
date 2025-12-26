use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T> ArcSwapOption<T> {
    /// A const-fn equivalent of [empty].
    ///
    /// Just like [empty], this creates an `None`-holding `ArcSwapOption`. The [empty] is, however,
    /// more general ‒ this is available only for the default strategy, while [empty] is for any
    /// [Default]-constructible strategy (current or future one).
    ///
    /// [empty]: ArcSwapAny::empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::sync::Arc;
    /// # use arc_swap::ArcSwapOption;
    /// static GLOBAL_DATA: ArcSwapOption<usize> = ArcSwapOption::const_empty();
    ///
    /// assert!(GLOBAL_DATA.load().is_none());
    /// GLOBAL_DATA.store(Some(Arc::new(42)));
    /// assert_eq!(42, **GLOBAL_DATA.load().as_ref().unwrap());
    /// ```
    pub const fn const_empty() -> Self {
        Self {
            ptr: AtomicPtr::new(ptr::null_mut()),
            _phantom_arc: PhantomData,
            strategy: HybridStrategy {
                _config: DefaultConfig,
            },
        }
    }
}
