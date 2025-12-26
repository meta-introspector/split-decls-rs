use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An owned reference to an occupied entry in a [`Slab`].
///
/// While the guard exists, it indicates to the slab that the item the guard
/// references is currently being accessed. If the item is removed from the slab
/// while the guard exists, the  removal will be deferred until all guards are
/// dropped.
///
/// Unlike [`Entry`], which borrows the slab, an `OwnedEntry` clones the [`Arc`]
/// around the slab. Therefore, it keeps the slab from being dropped until all
/// such guards have been dropped. This means that an `OwnedEntry` may be held for
/// an arbitrary lifetime.
///
/// # Examples
///
/// ```
/// # use sharded_slab::Slab;
/// use std::sync::Arc;
///
/// let slab: Arc<Slab<&'static str>> = Arc::new(Slab::new());
/// let key = slab.insert("hello world").unwrap();
///
/// // Look up the created key, returning an `OwnedEntry`.
/// let value = slab.clone().get_owned(key).unwrap();
///
/// // Now, the original `Arc` clone of the slab may be dropped, but the
/// // returned `OwnedEntry` can still access the value.
/// assert_eq!(value, "hello world");
/// ```
///
/// Unlike [`Entry`], an `OwnedEntry` may be stored in a struct which must live
/// for the `'static` lifetime:
///
/// ```
/// # use sharded_slab::Slab;
/// use sharded_slab::OwnedEntry;
/// use std::sync::Arc;
///
/// pub struct MyStruct {
///     entry: OwnedEntry<&'static str>,
///     // ... other fields ...
/// }
///
/// // Suppose this is some arbitrary function which requires a value that
/// // lives for the 'static lifetime...
/// fn function_requiring_static<T: 'static>(t: &T) {
///     // ... do something extremely important and interesting ...
/// }
///
/// let slab: Arc<Slab<&'static str>> = Arc::new(Slab::new());
/// let key = slab.insert("hello world").unwrap();
///
/// // Look up the created key, returning an `OwnedEntry`.
/// let entry = slab.clone().get_owned(key).unwrap();
/// let my_struct = MyStruct {
///     entry,
///     // ...
/// };
///
/// // We can use `my_struct` anywhere where it is required to have the
/// // `'static` lifetime:
/// function_requiring_static(&my_struct);
/// ```
///
/// `OwnedEntry`s may be sent between threads:
///
/// ```
/// # use sharded_slab::Slab;
/// use std::{thread, sync::Arc};
///
/// let slab: Arc<Slab<&'static str>> = Arc::new(Slab::new());
/// let key = slab.insert("hello world").unwrap();
///
/// // Look up the created key, returning an `OwnedEntry`.
/// let value = slab.clone().get_owned(key).unwrap();
///
/// thread::spawn(move || {
///     assert_eq!(value, "hello world");
///     // ...
/// }).join().unwrap();
/// ```
///
/// [`get`]: Slab::get
/// [`Arc`]: std::sync::Arc
pub struct OwnedEntry<T, C = DefaultConfig>
where
    C: cfg::Config,
{
    inner: page::slot::Guard<Option<T>, C>,
    value: ptr::NonNull<T>,
    slab: Arc<Slab<T, C>>,
    key: usize,
}
