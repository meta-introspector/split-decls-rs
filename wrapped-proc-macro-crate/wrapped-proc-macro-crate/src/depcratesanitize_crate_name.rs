// Generated macro for sanitize_crate_name (function)
macro_rules! Depcratesanitize_crate_name {
() => {
// Module: crate
// Provides: {"sanitize_crate_name"}
// Dependencies: {}
# [doc = " Make sure that the given crate name is a valid rust identifier."] fn sanitize_crate_name < S : AsRef < str > > (name : S) -> String { name . as_ref () . replace ('-' , "_") }
};
}
