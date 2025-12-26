use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Position in which `impl Trait` is disallowed.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ImplTraitPosition {
    Path,
    Variable,
    Trait,
    Bound,
    Generic,
    ExternFnParam,
    ClosureParam,
    PointerParam,
    FnTraitParam,
    ExternFnReturn,
    ClosureReturn,
    PointerReturn,
    FnTraitReturn,
    GenericDefault,
    ConstTy,
    StaticTy,
    AssocTy,
    FieldTy,
    Cast,
    ImplSelf,
    OffsetOf,
}
