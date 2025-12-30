// Generated macro for impl_108 (impl)
macro_rules! Depcrate_semanticsimpl_108 {
() => {
// Module: crate::semantics
// Provides: {"impl_108"}
// Dependencies: {}
impl < 'db > TypeInfo < 'db > { pub fn original (self) -> Type < 'db > { self . original } pub fn has_adjustment (& self) -> bool { self . adjusted . is_some () } # [doc = " The adjusted type, or the original in case no adjustments occurred."] pub fn adjusted (self) -> Type < 'db > { self . adjusted . unwrap_or (self . original) } }
};
}
