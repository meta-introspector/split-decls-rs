// Generated macro for impl_95 (impl)
macro_rules! Depcrate_value_ordimpl_95 {
() => {
// Module: crate::value_ord
// Provides: {"impl_95"}
// Dependencies: {}
impl DeriveValueOrd { # [doc = " Parse [`DeriveInput`]."] pub fn new (input : DeriveInput) -> syn :: Result < Self > { let ident = input . ident ; let type_attrs = TypeAttrs :: parse (& input . attrs) ? ; let (fields , input_type) = match input . data { syn :: Data :: Enum (data) => (data . variants . into_iter () . map (| variant | ValueField :: new_enum (variant , & type_attrs)) . collect :: < syn :: Result < _ > > () ? , InputType :: Enum ,) , syn :: Data :: Struct (data) => (data . fields . into_iter () . map (| field | ValueField :: new_struct (field , & type_attrs)) . collect :: < syn :: Result < _ > > () ? , InputType :: Struct ,) , _ => abort ! (ident , "can't derive `ValueOrd` on this type: \
                 only `enum` and `struct` types are allowed" ,) , } ; Ok (Self { ident , generics : input . generics . clone () , fields , input_type , }) } # [doc = " Lower the derived output into a [`TokenStream`]."] pub fn to_tokens (& self) -> TokenStream { let ident = & self . ident ; let (impl_generics , ty_generics , where_clause) = self . generics . split_for_impl () ; let mut body = Vec :: new () ; for field in & self . fields { body . push (field . to_tokens ()) ; } let body = match self . input_type { InputType :: Enum => { quote ! { # [allow (unused_imports)] use :: der :: ValueOrd ; match (self , other) { # (# body) * _ => unreachable ! () , } } } InputType :: Struct => { quote ! { # [allow (unused_imports)] use :: der :: { DerOrd , ValueOrd } ; # (# body) * Ok (:: core :: cmp :: Ordering :: Equal) } } } ; quote ! { impl # impl_generics :: der :: ValueOrd for # ident # ty_generics # where_clause { fn value_cmp (& self , other : & Self) -> :: der :: Result <:: core :: cmp :: Ordering > { # body } } } } }
};
}
