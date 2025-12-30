// Generated macro for prepare_cached_and_bind (macro)
macro_rules! Depcrateprepare_cached_and_bind {
() => {
// Module: crate
// Provides: {"prepare_cached_and_bind"}
// Dependencies: {}
# [doc = " Captured identifiers in SQL"] # [doc = ""] # [doc = " * only SQLite `$x` / `@x` / `:x` syntax works (Rust `&x` syntax does not"] # [doc = "   work)."] # [doc = " * `$x.y` expression does not work."] # [cfg (feature = "rusqlite-macros")] # [macro_export] macro_rules ! prepare_cached_and_bind { ($ conn : expr , $ sql : literal) => { { let mut stmt = $ conn . prepare_cached ($ sql) ?; $ crate :: __bind ! (stmt $ sql) ; stmt } } ; }
};
}
