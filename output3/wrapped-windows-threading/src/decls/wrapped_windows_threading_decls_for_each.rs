use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Calls the closure on each element of the iterator in parallel, waiting for all closures to finish.
///
/// * The closure does not require `'static` lifetime since the `for_each` function bounds the lifetime of all submitted closures.
/// * The closure must be `Sync` as multiple threads will refer to it.
/// * The iterator items must be `Send` as they will be sent from one thread to another.
pub fn for_each<I, F, T>(i: I, f: F)
where
    I: Iterator<Item = T>,
    F: Fn(T) + Sync,
    T: Send,
{
    Pool::with_scope(|pool| {
        for item in i {
            pool.submit(|| f(item));
        }
    });
}
