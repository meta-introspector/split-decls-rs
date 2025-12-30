// Generated macro for clone_trait_object (macro)
macro_rules! Depcrate_macrosclone_trait_object {
() => {
// Module: crate::macros
// Provides: {"clone_trait_object"}
// Dependencies: {}
# [doc = " Implement the standard library `Clone` for a trait object that has"] # [doc = " `DynClone` as a supertrait."] # [doc = ""] # [doc = " ```"] # [doc = " use dyn_clone::DynClone;"] # [doc = ""] # [doc = " trait MyTrait: DynClone {"] # [doc = "     /* ... */"] # [doc = " }"] # [doc = ""] # [doc = " dyn_clone::clone_trait_object!(MyTrait);"] # [doc = ""] # [doc = " // Now data structures containing Box<dyn MyTrait> can derive Clone."] # [doc = " #[derive(Clone)]"] # [doc = " struct Container {"] # [doc = "     trait_object: Box<dyn MyTrait>,"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " The macro supports traits that have type parameters and/or `where` clauses."] # [doc = ""] # [doc = " ```"] # [doc = " use dyn_clone::DynClone;"] # [doc = " use std::io::Read;"] # [doc = ""] # [doc = " trait Difficult<R>: DynClone where R: Read {"] # [doc = "     /* ... */"] # [doc = " }"] # [doc = ""] # [doc = " dyn_clone::clone_trait_object!(<R> Difficult<R> where R: Read);"] # [doc = " ```"] # [macro_export] macro_rules ! clone_trait_object { ($ ($ path : tt) +) => { $ crate :: __internal_clone_trait_object ! (begin $ ($ path) +) ; } ; }
};
}
