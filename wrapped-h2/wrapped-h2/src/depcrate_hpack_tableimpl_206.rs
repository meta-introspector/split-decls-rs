// Generated macro for impl_206 (impl)
macro_rules! Depcrate_hpack_tableimpl_206 {
() => {
// Module: crate::hpack::table
// Provides: {"impl_206"}
// Dependencies: {}
impl Index { fn new (v : Option < (usize , bool) > , e : Header) -> Index { match v { None => Index :: NotIndexed (e) , Some ((n , true)) => Index :: Indexed (n , e) , Some ((n , false)) => Index :: Name (n , e) , } } }
};
}
