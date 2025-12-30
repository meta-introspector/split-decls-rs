// Generated macro for impl_18 (impl)
macro_rules! Depcrate_utilsimpl_18 {
() => {
// Module: crate::utils
// Provides: {"impl_18"}
// Dependencies: {}
impl SliceExt for [Attribute] { # [doc = " # Errors"] # [doc = ""] # [doc = " - There are multiple specified attributes."] # [doc = " - The `Attribute::tokens` field of the specified attribute is not empty."] fn position_exact (& self , ident : & str) -> Result < Option < usize > > { self . iter () . try_fold ((0 , None) , | (i , mut prev) , attr | { if attr . path () . is_ident (ident) { if prev . replace (i) . is_some () { bail ! (attr , "duplicate #[{}] attribute" , ident) ; } attr . meta . require_path_only () ? ; } Ok ((i + 1 , prev)) }) . map (| (_ , pos) | pos) } fn find (& self , ident : & str) -> Option < & Attribute > { self . iter () . position (| attr | attr . path () . is_ident (ident)) . map (| i | & self [i]) } }
};
}
