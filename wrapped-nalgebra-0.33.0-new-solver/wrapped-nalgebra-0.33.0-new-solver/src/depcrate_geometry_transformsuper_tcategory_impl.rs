// Generated macro for super_tcategory_impl (macro)
macro_rules! Depcrate_geometry_transformsuper_tcategory_impl {
() => {
// Module: crate::geometry::transform
// Provides: {"super_tcategory_impl"}
// Dependencies: {}
macro_rules ! super_tcategory_impl (($ ($ a : ident >= $ b : ident) ;* $ (;) *) => { $ (impl SuperTCategoryOf <$ b > for $ a { }) * }) ;
};
}
