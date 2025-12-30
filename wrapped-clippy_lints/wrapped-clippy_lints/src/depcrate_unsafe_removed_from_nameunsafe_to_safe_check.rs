// Generated macro for unsafe_to_safe_check (function)
macro_rules! Depcrate_unsafe_removed_from_nameunsafe_to_safe_check {
() => {
// Module: crate::unsafe_removed_from_name
// Provides: {"unsafe_to_safe_check"}
// Dependencies: {}
fn unsafe_to_safe_check (old_name : Ident , new_name : Ident , cx : & EarlyContext < '_ > , span : Span) { let old_str = old_name . name . as_str () ; let new_str = new_name . name . as_str () ; if contains_unsafe (old_str) && ! contains_unsafe (new_str) { span_lint (cx , UNSAFE_REMOVED_FROM_NAME , span , format ! ("removed `unsafe` from the name of `{old_str}` in use as `{new_str}`") ,) ; } }
};
}
