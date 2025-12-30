// Generated macro for impl_170 (impl)
macro_rules! Depcrate_columnimpl_170 {
() => {
// Module: crate::column
// Provides: {"impl_170"}
// Dependencies: {}
# [cfg (feature = "column_decltype")] impl Column < '_ > { # [doc = " Returns the name of the column."] # [inline] # [must_use] pub fn name (& self) -> & str { self . name } # [doc = " Returns the type of the column (`None` for expression)."] # [inline] # [must_use] pub fn decl_type (& self) -> Option < & str > { self . decl_type } }
};
}
