// Generated macro for check_and_apply_linkage (function)
macro_rules! Depcrate_constscheck_and_apply_linkage {
() => {
// Module: crate::consts
// Provides: {"check_and_apply_linkage"}
// Dependencies: {}
fn check_and_apply_linkage < 'gcc , 'tcx > (cx : & CodegenCx < 'gcc , 'tcx > , attrs : & CodegenFnAttrs , gcc_type : Type < 'gcc > , sym : & str ,) -> LValue < 'gcc > { let is_tls = attrs . flags . contains (CodegenFnAttrFlags :: THREAD_LOCAL) ; if let Some (linkage) = attrs . import_linkage { let global1 = cx . declare_global_with_linkage (sym , cx . type_i8 () , base :: global_linkage_to_gcc (linkage)) ; if linkage == Linkage :: ExternalWeak { # [cfg (feature = "master")] global1 . add_attribute (VarAttribute :: Weak) ; } let real_name = format ! ("_rust_extern_with_linkage_{:016x}_{sym}" , cx . tcx . stable_crate_id (LOCAL_CRATE)) ; let global2 = cx . define_global (& real_name , gcc_type , is_tls , attrs . link_section) ; let value = cx . const_ptrcast (global1 . get_address (None) , gcc_type) ; global2 . global_set_initializer_rvalue (value) ; global2 } else { cx . declare_global (sym , gcc_type , GlobalKind :: Imported , is_tls , attrs . link_section) } }
};
}
