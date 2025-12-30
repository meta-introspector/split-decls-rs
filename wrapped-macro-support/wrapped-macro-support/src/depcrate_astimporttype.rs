// Generated macro for ImportType (struct)
macro_rules! Depcrate_astImportType {
() => {
// Module: crate::ast
// Provides: {"ImportType"}
// Dependencies: {}
# [doc = " The metadata for a type being imported"] # [cfg_attr (feature = "extra-traits" , derive (Debug , PartialEq , Eq))] # [derive (Clone)] pub struct ImportType { # [doc = " The visibility of this type in Rust"] pub vis : syn :: Visibility , # [doc = " The name of this type on the Rust side"] pub rust_name : Ident , # [doc = " The name of this type on the JS side"] pub js_name : String , # [doc = " The custom attributes to apply to this type"] pub attrs : Vec < syn :: Attribute > , # [doc = " The TS definition to generate for this type"] pub typescript_type : Option < String > , # [doc = " The doc comment applied to this type, if one exists"] pub doc_comment : Option < String > , # [doc = " The name of the shim to check instanceof for this type"] pub instanceof_shim : String , # [doc = " The name of the remote function to use for the generated is_type_of"] pub is_type_of : Option < syn :: Expr > , # [doc = " The list of classes this extends, if any"] pub extends : Vec < syn :: Path > , # [doc = " A custom prefix to add and attempt to fall back to, if the type isn't found"] pub vendor_prefixes : Vec < Ident > , # [doc = " If present, don't generate a `Deref` impl"] pub no_deref : bool , # [doc = " Path to wasm_bindgen"] pub wasm_bindgen : Path , }
};
}
