// Generated macro for impl_46 (impl)
macro_rules! Depcrateimpl_46 {
() => {
// Module: crate
// Provides: {"impl_46"}
// Dependencies: {}
impl GenerationConfiguration { fn get (& self , key : & str) -> Option < & TypeGeneration > { self . map . get (key) . or_else (| | { self . generate_by_default . then_some (& TypeGeneration :: Generate) }) } fn insert (& mut self , name : String , generate : TypeGeneration) { self . map . insert (name , generate) ; } fn iter (& self) -> impl Iterator < Item = (& String , & TypeGeneration) > { self . map . iter () } }
};
}
