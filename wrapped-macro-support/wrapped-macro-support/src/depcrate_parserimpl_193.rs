// Generated macro for impl_193 (impl)
macro_rules! Depcrate_parserimpl_193 {
() => {
// Module: crate::parser
// Provides: {"impl_193"}
// Dependencies: {}
impl ConvertToAst < (& ast :: Program , BindgenAttrs) > for syn :: ForeignItemType { type Target = ast :: ImportKind ; fn convert (self , (program , attrs) : (& ast :: Program , BindgenAttrs) ,) -> Result < Self :: Target , Diagnostic > { let js_name = attrs . js_name () . map (| s | s . 0) . map_or_else (| | self . ident . to_string () , | s | s . to_string ()) ; let typescript_type = attrs . typescript_type () . map (| s | s . 0 . to_string ()) ; let is_type_of = attrs . is_type_of () . cloned () ; let shim = format ! ("__wbg_instanceof_{}_{}" , self . ident , ShortHash ((attrs . js_namespace () . map (| (ns , _) | ns . 0) , & self . ident))) ; let mut extends = Vec :: new () ; let mut vendor_prefixes = Vec :: new () ; let no_deref = attrs . no_deref () . is_some () ; for (used , attr) in attrs . attrs . iter () { match attr { BindgenAttr :: Extends (_ , e) => { extends . push (e . clone ()) ; used . set (true) ; } BindgenAttr :: VendorPrefix (_ , e) => { vendor_prefixes . push (e . clone ()) ; used . set (true) ; } _ => { } } } attrs . check_used () ; Ok (ast :: ImportKind :: Type (ast :: ImportType { vis : self . vis , attrs : self . attrs , doc_comment : None , instanceof_shim : shim , is_type_of , rust_name : self . ident , typescript_type , js_name , extends , vendor_prefixes , no_deref , wasm_bindgen : program . wasm_bindgen . clone () , })) } }
};
}
