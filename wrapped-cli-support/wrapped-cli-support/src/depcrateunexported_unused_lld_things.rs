// Generated macro for unexported_unused_lld_things (function)
macro_rules! Depcrateunexported_unused_lld_things {
() => {
// Module: crate
// Provides: {"unexported_unused_lld_things"}
// Dependencies: {}
# [doc = " Remove a number of internal exports that are synthesized by Rust's linker,"] # [doc = " LLD. These exports aren't typically ever needed and just add extra space to"] # [doc = " the binary."] fn unexported_unused_lld_things (module : & mut Module) { let mut to_remove = Vec :: new () ; for export in module . exports . iter () { match export . name . as_str () { "__heap_base" | "__data_end" | "__indirect_function_table" => { to_remove . push (export . id ()) ; } _ => { } } } for id in to_remove { module . exports . delete (id) ; } }
};
}
