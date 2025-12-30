// Generated macro for impl_915 (impl)
macro_rules! Depcrate_core_builderimpl_915 {
() => {
// Module: crate::core::builder
// Provides: {"impl_915"}
// Dependencies: {}
impl StepMetadata { pub fn build (name : & str , target : TargetSelection) -> Self { Self :: new (name , target , Kind :: Build) } pub fn check (name : & str , target : TargetSelection) -> Self { Self :: new (name , target , Kind :: Check) } pub fn clippy (name : & str , target : TargetSelection) -> Self { Self :: new (name , target , Kind :: Clippy) } pub fn doc (name : & str , target : TargetSelection) -> Self { Self :: new (name , target , Kind :: Doc) } pub fn dist (name : & str , target : TargetSelection) -> Self { Self :: new (name , target , Kind :: Dist) } pub fn test (name : & str , target : TargetSelection) -> Self { Self :: new (name , target , Kind :: Test) } pub fn run (name : & str , target : TargetSelection) -> Self { Self :: new (name , target , Kind :: Run) } fn new (name : & str , target : TargetSelection , kind : Kind) -> Self { Self { name : name . to_string () , kind , target , built_by : None , stage : None , metadata : None } } pub fn built_by (mut self , compiler : Compiler) -> Self { self . built_by = Some (compiler) ; self } pub fn stage (mut self , stage : u32) -> Self { self . stage = Some (stage) ; self } pub fn with_metadata (mut self , metadata : String) -> Self { self . metadata = Some (metadata) ; self } pub fn get_stage (& self) -> Option < u32 > { self . stage . or (self . built_by . map (| compiler | if self . name == "std" { compiler . stage } else { compiler . stage + 1 })) } pub fn get_name (& self) -> & str { & self . name } pub fn get_target (& self) -> TargetSelection { self . target } }
};
}
