// Generated macro for is_name_ref_in_test (function)
macro_rules! Depcrate_searchis_name_ref_in_test {
() => {
// Module: crate::search
// Provides: {"is_name_ref_in_test"}
// Dependencies: {}
fn is_name_ref_in_test (sema : & Semantics < '_ , RootDatabase > , name_ref : & ast :: NameRef) -> bool { name_ref . syntax () . ancestors () . any (| node | match ast :: Fn :: cast (node) { Some (it) => sema . to_def (& it) . is_some_and (| func | func . is_test (sema . db)) , None => false , }) }
};
}
