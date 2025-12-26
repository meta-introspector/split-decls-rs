use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A future, stream, sink, or executor that has been instrumented with a
/// `tracing` subscriber.
#[cfg(all(feature = "std", not(feature = "std-future")))]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[derive(Clone, Debug)]
pub struct WithDispatch<T> {
    inner: T,
    dispatch: Dispatch,
}
