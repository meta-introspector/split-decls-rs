use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An iterator that fails with a predetermined error exactly once.
#[derive(Clone, Debug)]
pub struct OnceErr<T, E>(PhantomData<T>, Option<E>);
