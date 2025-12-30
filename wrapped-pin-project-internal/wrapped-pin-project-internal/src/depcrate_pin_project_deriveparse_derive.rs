// Generated macro for parse_derive (function)
macro_rules! Depcrate_pin_project_deriveparse_derive {
() => {
// Module: crate::pin_project::derive
// Provides: {"parse_derive"}
// Dependencies: {}
pub (super) fn parse_derive (input : TokenStream) -> Result < TokenStream > { let mut input : Item = syn :: parse2 (input) ? ; let mut cx ; let mut generate = GenerateTokens :: default () ; match & mut input { Item :: Struct (input) => { let ident = & input . ident ; let ty_generics = input . generics . split_for_impl () . 1 ; let self_ty = parse_quote ! (# ident # ty_generics) ; let mut visitor = ReplaceReceiver (& self_ty) ; visitor . visit_item_struct_mut (input) ; cx = Context :: new (& input . attrs , & input . vis , & input . ident , & mut input . generics , Struct) ? ; parse_struct (& mut cx , & input . fields , & mut generate) ? ; } Item :: Enum (input) => { let ident = & input . ident ; let ty_generics = input . generics . split_for_impl () . 1 ; let self_ty = parse_quote ! (# ident # ty_generics) ; let mut visitor = ReplaceReceiver (& self_ty) ; visitor . visit_item_enum_mut (input) ; cx = Context :: new (& input . attrs , & input . vis , & input . ident , & mut input . generics , Enum) ? ; parse_enum (& mut cx , input . brace_token , & input . variants , & mut generate) ? ; } _ => bail ! (input , "#[pin_project] attribute may only be used on structs or enums") , } Ok (generate . into_tokens (& cx)) }
};
}
