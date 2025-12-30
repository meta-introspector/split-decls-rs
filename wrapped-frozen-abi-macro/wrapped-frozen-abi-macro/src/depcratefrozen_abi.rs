// Generated macro for frozen_abi (function)
macro_rules! Depcratefrozen_abi {
() => {
// Module: crate
// Provides: {"frozen_abi"}
// Dependencies: {}
# [cfg (feature = "frozen-abi")] # [proc_macro_attribute] pub fn frozen_abi (attrs : TokenStream , item : TokenStream) -> TokenStream { let mut api_expected_digest : Option < String > = None ; let mut abi_expected_digest : Option < String > = None ; let attrs_parser = syn :: meta :: parser (| meta | { if meta . path . is_ident ("digest") || meta . path . is_ident ("api_digest") { api_expected_digest = Some (meta . value () ? . parse :: < LitStr > () ? . value ()) ; Ok (()) } else if meta . path . is_ident ("abi_digest") { abi_expected_digest = Some (meta . value () ? . parse :: < LitStr > () ? . value ()) ; Ok (()) } else { Err (meta . error ("unsupported \"frozen_abi\" property")) } }) ; parse_macro_input ! (attrs with attrs_parser) ; let Some (api_expected_digest) = api_expected_digest else { return Error :: new_spanned (TokenStream2 :: from (item) , "missing required attribute: #[frozen_abi(api_digest = \"...\")]" ,) . to_compile_error () . into () ; } ; let item = parse_macro_input ! (item as Item) ; match item { Item :: Struct (input) => { frozen_abi_struct_type (input , & api_expected_digest , abi_expected_digest . as_deref ()) } Item :: Enum (input) => { frozen_abi_enum_type (input , & api_expected_digest , abi_expected_digest . as_deref ()) } Item :: Type (input) => { frozen_abi_type_alias (input , & api_expected_digest , abi_expected_digest . as_deref ()) } _ => Error :: new_spanned (item , "frozen_abi isn't applicable; only for struct, enum and type" ,) . to_compile_error () . into () , } }
};
}
