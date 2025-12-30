// Generated macro for impl_1196 (impl)
macro_rules! Depcrate_type_ofimpl_1196 {
() => {
// Module: crate::type_of
// Provides: {"impl_1196"}
// Dependencies: {}
impl < 'a , 'tcx > CodegenCx < 'a , 'tcx > { pub (crate) fn align_of (& self , ty : Ty < 'tcx >) -> Align { self . layout_of (ty) . align . abi } pub (crate) fn size_of (& self , ty : Ty < 'tcx >) -> Size { self . layout_of (ty) . size } pub (crate) fn size_and_align_of (& self , ty : Ty < 'tcx >) -> (Size , Align) { self . spanned_size_and_align_of (ty , DUMMY_SP) } pub (crate) fn spanned_size_and_align_of (& self , ty : Ty < 'tcx > , span : Span) -> (Size , Align) { let layout = self . spanned_layout_of (ty , span) ; (layout . size , layout . align . abi) } }
};
}
