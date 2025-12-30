// Generated macro for impl_98 (impl)
macro_rules! Depcrate_value_ordimpl_98 {
() => {
// Module: crate::value_ord
// Provides: {"impl_98"}
// Dependencies: {}
impl ValueField { # [doc = " Create from an `enum` variant."] fn new_enum (variant : Variant , type_attrs : & TypeAttrs) -> syn :: Result < Self > { let ident = variant . ident ; let attrs = FieldAttrs :: parse (& variant . attrs , type_attrs) ? ; Ok (Self { ident , attrs , is_enum : true , }) } # [doc = " Create from a `struct` field."] fn new_struct (field : Field , type_attrs : & TypeAttrs) -> syn :: Result < Self > { let ident = field . ident . as_ref () . cloned () . ok_or_else (| | { syn :: Error :: new_spanned (& field , "tuple structs are not supported") }) ? ; let attrs = FieldAttrs :: parse (& field . attrs , type_attrs) ? ; Ok (Self { ident , attrs , is_enum : false , }) } # [doc = " Lower to [`TokenStream`]."] fn to_tokens (& self) -> TokenStream { let ident = & self . ident ; if self . is_enum { let binding1 = quote ! (Self ::# ident (this)) ; let binding2 = quote ! (Self ::# ident (other)) ; quote ! { (# binding1 , # binding2) => this . value_cmp (other) , } } else { let mut binding1 = quote ! (self .# ident) ; let mut binding2 = quote ! (other .# ident) ; if let Some (ty) = & self . attrs . asn1_type { binding1 = ty . encoder (& binding1) ; binding2 = ty . encoder (& binding2) ; } quote ! { match # binding1 . der_cmp (&# binding2) ? { :: core :: cmp :: Ordering :: Equal => () , other => return Ok (other) , } } } } }
};
}
