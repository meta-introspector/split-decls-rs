use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemKind<'tcx> {
    Struct(PhantomData<&'tcx ()>),
    Enum(PhantomData<&'tcx ()>),
    Union(PhantomData<&'tcx ()>),
}
