// Generated macro for impl_642 (impl)
macro_rules! Depcrate_types_to_sqlimpl_642 {
() => {
// Module: crate::types::to_sql
// Provides: {"impl_642"}
// Dependencies: {}
impl < 'a , T : ? Sized > From < & 'a T > for ToSqlOutput < 'a > where & 'a T : Into < ValueRef < 'a > > , { # [inline] fn from (t : & 'a T) -> Self { ToSqlOutput :: Borrowed (t . into ()) } }
};
}
