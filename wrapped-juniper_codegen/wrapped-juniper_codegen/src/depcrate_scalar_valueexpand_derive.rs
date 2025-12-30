// Generated macro for expand_derive (function)
macro_rules! Depcrate_scalar_valueexpand_derive {
() => {
// Module: crate::scalar_value
// Provides: {"expand_derive"}
// Dependencies: {}
# [doc = " Expands `#[derive(ScalarValue)]` macro into generated code."] pub fn expand_derive (input : TokenStream) -> syn :: Result < TokenStream > { let ast = syn :: parse2 :: < syn :: DeriveInput > (input) ? ; let data_enum = match ast . data { syn :: Data :: Enum (e) => e , _ => return Err (ERR . custom_error (ast . span () , "can only be derived for enums")) , } ; let attr = Attr :: from_attrs ("value" , & ast . attrs) ? ; let mut methods = HashMap :: < Method , Vec < Variant > > :: new () ; for var in data_enum . variants . clone () { let (ident , field) = (var . ident , Field :: try_from (var . fields) ?) ; for attr in VariantAttr :: from_attrs ("value" , & var . attrs) ? . 0 { let (method , expr) = attr . into_inner () ; methods . entry (method) . or_default () . push (Variant { ident : ident . clone () , field : field . clone () , expr , }) ; } } Ok (Definition { ident : ast . ident , generics : ast . generics , variants : data_enum . variants . into_iter () . collect () , methods , from_displayable : attr . from_displayable . map (SpanContainer :: into_inner) , from_displayable_non_static : attr . from_displayable_non_static . map (SpanContainer :: into_inner) , } . into_token_stream ()) }
};
}
