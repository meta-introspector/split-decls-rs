// Generated macro for match_libc_symbol (function)
macro_rules! Depcratematch_libc_symbol {
() => {
// Module: crate
// Provides: {"match_libc_symbol"}
// Dependencies: {}
# [doc = " Checks if the given `DefId` matches the `libc` item."] pub fn match_libc_symbol (cx : & LateContext < '_ > , did : DefId , name : Symbol) -> bool { cx . tcx . crate_name (did . krate) == sym :: libc && cx . tcx . def_path_str (did) . ends_with (name . as_str ()) }
};
}
