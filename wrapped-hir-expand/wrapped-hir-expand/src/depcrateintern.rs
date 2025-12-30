// Generated macro for Intern (trait)
macro_rules! DepcrateIntern {
() => {
// Module: crate
// Provides: {"Intern"}
// Dependencies: {}
pub trait Intern { type Database : ? Sized ; type ID ; fn intern (self , db : & Self :: Database) -> Self :: ID ; }
};
}
