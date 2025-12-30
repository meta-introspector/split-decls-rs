// Generated macro for Lookup (trait)
macro_rules! DepcrateLookup {
() => {
// Module: crate
// Provides: {"Lookup"}
// Dependencies: {}
pub trait Lookup { type Database : ? Sized ; type Data ; fn lookup (& self , db : & Self :: Database) -> Self :: Data ; }
};
}
