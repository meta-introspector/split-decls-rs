// Generated macro for expand_struct_marker (function)
macro_rules! Depcrateexpand_struct_marker {
() => {
// Module: crate
// Provides: {"expand_struct_marker"}
// Dependencies: {}
pub fn expand_struct_marker (item : TokenStream) -> Result < TokenStream , Diagnostic > { parser :: reset_attrs_used () ; let mut s : syn :: ItemStruct = syn :: parse2 (item) ? ; let mut program = ast :: Program :: default () ; program . structs . push ((& mut s) . convert (& program) ?) ; let mut tokens = proc_macro2 :: TokenStream :: new () ; program . try_to_tokens (& mut tokens) ? ; parser :: check_unused_attrs (& mut tokens) ; Ok (tokens) }
};
}
