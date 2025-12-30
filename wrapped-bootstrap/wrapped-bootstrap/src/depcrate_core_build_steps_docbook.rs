// Generated macro for book (macro)
macro_rules! Depcrate_core_build_steps_docbook {
() => {
// Module: crate::core::build_steps::doc
// Provides: {"book"}
// Dependencies: {}
macro_rules ! book { ($ ($ name : ident , $ path : expr , $ book_name : expr , $ lang : expr ;) +) => { $ (# [derive (Debug , Clone , Hash , PartialEq , Eq)] pub struct $ name { target : TargetSelection , } impl Step for $ name { type Output = () ; const DEFAULT : bool = true ; fn should_run (run : ShouldRun <'_ >) -> ShouldRun <'_ > { let builder = run . builder ; run . path ($ path) . default_condition (builder . config . docs) } fn make_run (run : RunConfig <'_ >) { run . builder . ensure ($ name { target : run . target , }) ; } fn run (self , builder : & Builder <'_ >) { if let Some (submodule_path) = submodule_path_of (& builder , $ path) { builder . require_submodule (& submodule_path , None) } builder . ensure (RustbookSrc { target : self . target , name : $ book_name . to_owned () , src : builder . src . join ($ path) , parent : Some (self) , languages : $ lang . into () , build_compiler : None , }) } }) + } }
};
}
