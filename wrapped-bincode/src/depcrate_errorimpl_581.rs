// Generated macro for impl_581 (impl)
macro_rules! Depcrate_errorimpl_581 {
() => {
// Module: crate::error
// Provides: {"impl_581"}
// Dependencies: {}
impl IntegerType { # [doc = " Change the `Ux` value to the associated `Ix` value."] # [doc = " Returns the old value if `self` is already `Ix`."] pub (crate) const fn into_signed (self) -> Self { match self { Self :: U8 => Self :: I8 , Self :: U16 => Self :: I16 , Self :: U32 => Self :: I32 , Self :: U64 => Self :: I64 , Self :: U128 => Self :: I128 , Self :: Usize => Self :: Isize , other => other , } } }
};
}
