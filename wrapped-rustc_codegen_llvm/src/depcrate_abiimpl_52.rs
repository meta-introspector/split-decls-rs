// Generated macro for impl_52 (impl)
macro_rules! Depcrate_abiimpl_52 {
() => {
// Module: crate::abi
// Provides: {"impl_52"}
// Dependencies: {}
impl ArgAttributesExt for ArgAttributes { fn apply_attrs_to_llfn (& self , idx : AttributePlace , cx : & CodegenCx < '_ , '_ > , llfn : & Value) { let attrs = get_attrs (self , cx) ; attributes :: apply_to_llfn (llfn , idx , & attrs) ; } fn apply_attrs_to_callsite (& self , idx : AttributePlace , cx : & CodegenCx < '_ , '_ > , callsite : & Value ,) { let attrs = get_attrs (self , cx) ; attributes :: apply_to_callsite (callsite , idx , & attrs) ; } }
};
}
