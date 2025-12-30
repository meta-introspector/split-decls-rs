// Generated macro for clean_crate_tree (macro)
macro_rules! Depcrate_core_build_steps_cleanclean_crate_tree {
() => {
// Module: crate::core::build_steps::clean
// Provides: {"clean_crate_tree"}
// Dependencies: {}
macro_rules ! clean_crate_tree { ($ ($ name : ident , $ mode : path , $ root_crate : literal) ;+ $ (;) ?) => { $ (# [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct $ name { compiler : Compiler , crates : Vec < String >, } impl Step for $ name { type Output = () ; fn should_run (run : ShouldRun <'_ >) -> ShouldRun <'_ > { let crates = run . builder . in_tree_crates ($ root_crate , None) ; run . crates (crates) } fn make_run (run : RunConfig <'_ >) { let builder = run . builder ; let compiler = builder . compiler (builder . top_stage , run . target) ; builder . ensure (Self { crates : run . cargo_crates_in_set () , compiler }) ; } fn run (self , builder : & Builder <'_ >) -> Self :: Output { let compiler = self . compiler ; let target = compiler . host ; let mut cargo = builder . bare_cargo (compiler , $ mode , target , Kind :: Clean) ; cargo . env ("RUSTC_BOOTSTRAP" , "1") ; for krate in &* self . crates { cargo . arg ("-p") ; cargo . arg (krate) ; } builder . info (& format ! ("Cleaning{} stage{} {} artifacts ({} -> {})" , crate_description (& self . crates) , compiler . stage , stringify ! ($ name) . to_lowercase () , & compiler . host , target ,)) ; cargo . run (builder) ; } }) + } }
};
}
