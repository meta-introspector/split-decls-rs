// Generated macro for BorshSchema (trait)
macro_rules! Depcrate_schemaBorshSchema {
() => {
// Module: crate::schema
// Provides: {"BorshSchema"}
// Dependencies: {}
# [doc = " The declaration and the definition of the type that can be used to (de)serialize Borsh without"] # [doc = " the Rust type that produced it."] pub trait BorshSchema { # [doc = " Recursively, using DFS, add type definitions required for this type."] # [doc = " Type definition partially explains how to serialize/deserialize a type."] fn add_definitions_recursively (definitions : & mut BTreeMap < Declaration , Definition >) ; # [doc = " Get the name of the type without brackets."] fn declaration () -> Declaration ; }
};
}
