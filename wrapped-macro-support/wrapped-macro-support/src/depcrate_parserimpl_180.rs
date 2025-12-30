// Generated macro for impl_180 (impl)
macro_rules! Depcrate_parserimpl_180 {
() => {
// Module: crate::parser
// Provides: {"impl_180"}
// Dependencies: {}
impl BindgenAttrs { # [doc = " Find and parse the wasm_bindgen attributes."] fn find (attrs : & mut Vec < syn :: Attribute >) -> Result < BindgenAttrs , Diagnostic > { let mut ret = BindgenAttrs :: default () ; loop { let pos = attrs . iter () . enumerate () . find (| & (_ , m) | m . path () . segments [0] . ident == "wasm_bindgen") . map (| a | a . 0) ; let pos = match pos { Some (i) => i , None => return Ok (ret) , } ; let attr = attrs . remove (pos) ; let tokens = match attr . meta { syn :: Meta :: Path (_) => continue , syn :: Meta :: List (syn :: MetaList { delimiter : MacroDelimiter :: Paren (_) , tokens , .. }) => tokens , syn :: Meta :: List (_) | syn :: Meta :: NameValue (_) => { bail_span ! (attr , "malformed #[wasm_bindgen] attribute") } } ; let mut attrs : BindgenAttrs = syn :: parse2 (tokens) ? ; ret . attrs . append (& mut attrs . attrs) ; attrs . check_used () ; } } fn get_thread_local (& self) -> Result < Option < ThreadLocal > , Diagnostic > { let mut thread_local = self . thread_local_v2 () . map (| _ | ThreadLocal :: V2) ; if let Some (span) = self . thread_local () { if thread_local . is_some () { return Err (Diagnostic :: span_error (* span , "`thread_local` can't be used with `thread_local_v2`" ,)) ; } else { thread_local = Some (ThreadLocal :: V1) } } Ok (thread_local) } attrgen ! (methods) ; }
};
}
