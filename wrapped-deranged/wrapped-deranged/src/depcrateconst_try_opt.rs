// Generated macro for const_try_opt (macro)
macro_rules! Depcrateconst_try_opt {
() => {
// Module: crate
// Provides: {"const_try_opt"}
// Dependencies: {}
# [doc = " `?` for `Option` types, usable in `const` contexts."] macro_rules ! const_try_opt { ($ e : expr) => { match $ e { Some (value) => value , None => return None , } } ; }
};
}
