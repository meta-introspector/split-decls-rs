use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug)]
pub struct Callable<'db> {
    ty: Type<'db>,
    sig: PolyFnSig<'db>,
    callee: Callee<'db>,
    /// Whether this is a method that was called with method call syntax.
    is_bound_method: bool,
}
