// Generated macro for impl_323 (impl)
macro_rules! Depcrate_tableimpl_323 {
() => {
// Module: crate::table
// Provides: {"impl_323"}
// Dependencies: {}
impl < 'a , Item > FromIterator < Item > for Table < 'a > where Item : Into < Row < 'a > > , { # [doc = " Collects an iterator of rows into a table."] # [doc = ""] # [doc = " When collecting from an iterator into a table, the user must provide the widths using"] # [doc = " `Table::widths` after construction."] fn from_iter < Iter : IntoIterator < Item = Item > > (rows : Iter) -> Self { let widths : [Constraint ; 0] = [] ; Self :: new (rows , widths) } }
};
}
