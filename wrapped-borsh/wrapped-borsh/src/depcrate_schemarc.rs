// Generated macro for rc (module)
macro_rules! Depcrate_schemarc {
() => {
// Module: crate::schema
// Provides: {"rc"}
// Dependencies: {}
# [doc = " Module is available if borsh is built with `features = [\"rc\"]`."] # [cfg (feature = "rc")] pub mod rc { # ! [doc = ""] # ! [doc = " Module defines [BorshSchema] implementation for"] # ! [doc = " [alloc::rc::Rc](std::rc::Rc) and [alloc::sync::Arc](std::sync::Arc)."] use crate :: BorshSchema ; use super :: { Declaration , Definition } ; use crate :: __private :: maybestd :: collections :: BTreeMap ; use crate :: __private :: maybestd :: { rc :: Rc , sync :: Arc } ; impl < T > BorshSchema for Rc < T > where T : BorshSchema + ? Sized , { fn add_definitions_recursively (definitions : & mut BTreeMap < Declaration , Definition >) { T :: add_definitions_recursively (definitions) ; } fn declaration () -> Declaration { T :: declaration () } } impl < T > BorshSchema for Arc < T > where T : BorshSchema + ? Sized , { fn add_definitions_recursively (definitions : & mut BTreeMap < Declaration , Definition >) { T :: add_definitions_recursively (definitions) ; } fn declaration () -> Declaration { T :: declaration () } } }
};
}
