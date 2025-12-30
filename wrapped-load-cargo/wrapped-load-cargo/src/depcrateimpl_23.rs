// Generated macro for impl_23 (impl)
macro_rules! Depcrateimpl_23 {
() => {
// Module: crate
// Provides: {"impl_23"}
// Dependencies: {}
impl ProcMacroExpander for Expander { fn expand (& self , subtree : & tt :: TopSubtree < Span > , attrs : Option < & tt :: TopSubtree < Span > > , env : & Env , def_site : Span , call_site : Span , mixed_site : Span , current_dir : String ,) -> Result < tt :: TopSubtree < Span > , ProcMacroExpansionError > { match self . 0 . expand (subtree . view () , attrs . map (| attrs | attrs . view ()) , env . clone () . into () , def_site , call_site , mixed_site , current_dir ,) { Ok (Ok (subtree)) => Ok (subtree) , Ok (Err (err)) => Err (ProcMacroExpansionError :: Panic (err)) , Err (err) => Err (ProcMacroExpansionError :: System (err . to_string ())) , } } fn eq_dyn (& self , other : & dyn ProcMacroExpander) -> bool { (other as & dyn Any) . downcast_ref :: < Self > () == Some (self) } }
};
}
