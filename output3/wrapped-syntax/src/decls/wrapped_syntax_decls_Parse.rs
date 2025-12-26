use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// `Parse` is the result of the parsing: a syntax tree and a collection of
/// errors.
///
/// Note that we always produce a syntax tree, even for completely invalid
/// files.
#[derive(Debug, PartialEq, Eq)]
pub struct Parse<T> {
    green: Option<GreenNode>,
    errors: Option<Arc<[SyntaxError]>>,
    _ty: PhantomData<fn() -> T>,
}
