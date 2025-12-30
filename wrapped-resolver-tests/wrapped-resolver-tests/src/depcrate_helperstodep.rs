// Generated macro for ToDep (trait)
macro_rules! Depcrate_helpersToDep {
() => {
// Module: crate::helpers
// Provides: {"ToDep"}
// Dependencies: {}
pub trait ToDep { fn to_dep (self) -> Dependency ; fn opt (self) -> Dependency ; fn with (self , features : & [& 'static str]) -> Dependency ; fn with_default (self) -> Dependency ; fn rename (self , name : & str) -> Dependency ; }
};
}
