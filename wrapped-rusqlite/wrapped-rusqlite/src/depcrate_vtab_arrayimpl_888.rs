// Generated macro for impl_888 (impl)
macro_rules! Depcrate_vtab_arrayimpl_888 {
() => {
// Module: crate::vtab::array
// Provides: {"impl_888"}
// Dependencies: {}
impl ArrayTabCursor < '_ > { fn new < 'vtab > () -> ArrayTabCursor < 'vtab > { ArrayTabCursor { base : ffi :: sqlite3_vtab_cursor :: default () , row_id : 0 , ptr : None , phantom : PhantomData , } } fn len (& self) -> i64 { match self . ptr { Some (ref a) => a . len () as i64 , _ => 0 , } } }
};
}
