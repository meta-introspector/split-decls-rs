// Generated macro for impl_107 (impl)
macro_rules! Depcrate_semanticsimpl_107 {
() => {
// Module: crate::semantics
// Provides: {"impl_107"}
// Dependencies: {}
impl < 'db > TypeInfo < 'db > { pub fn original (self) -> Type < 'db > { self . original } pub fn has_adjustment (& self) -> bool { self . adjusted . is_some () } # [doc = " The adjusted type, or the original in case no adjustments occurred."] pub fn adjusted (self) -> Type < 'db > { self . adjusted . unwrap_or (self . original) } }
};
}
