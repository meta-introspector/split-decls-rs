// Generated macro for impl_204 (impl)
macro_rules! Depcrate_parserimpl_204 {
() => {
// Module: crate::parser
// Provides: {"impl_204"}
// Dependencies: {}
impl MacroParse < BindgenAttrs > for & mut syn :: ItemImpl { fn macro_parse (self , program : & mut ast :: Program , opts : BindgenAttrs) -> Result < () , Diagnostic > { if self . defaultness . is_some () { bail_span ! (self . defaultness , "#[wasm_bindgen] default impls are not supported") ; } if self . unsafety . is_some () { bail_span ! (self . unsafety , "#[wasm_bindgen] unsafe impls are not supported") ; } if let Some ((_ , path , _)) = & self . trait_ { bail_span ! (path , "#[wasm_bindgen] trait impls are not supported") ; } if ! self . generics . params . is_empty () { bail_span ! (self . generics , "#[wasm_bindgen] generic impls aren't supported") ; } let name = match get_ty (& self . self_ty) { syn :: Type :: Path (syn :: TypePath { qself : None , ref path , }) => path , _ => bail_span ! (self . self_ty , "unsupported self type in #[wasm_bindgen] impl") , } ; let mut errors = Vec :: new () ; for item in self . items . iter_mut () { if let Err (e) = prepare_for_impl_recursion (item , name , program , & opts) { errors . push (e) ; } } Diagnostic :: from_vec (errors) ? ; opts . check_used () ; Ok (()) } }
};
}
