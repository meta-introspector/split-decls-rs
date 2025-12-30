// Generated macro for process (function)
macro_rules! Depcrate_internals_serialize_structsprocess {
() => {
// Module: crate::internals::serialize::structs
// Provides: {"process"}
// Dependencies: {}
pub fn process (input : & ItemStruct , cratename : Path) -> syn :: Result < TokenStream2 > { let name = & input . ident ; let generics = generics :: without_defaults (& input . generics) ; let (impl_generics , ty_generics , where_clause) = generics . split_for_impl () ; let mut where_clause = generics :: default_where (where_clause) ; let mut body = TokenStream2 :: new () ; let mut generics_output = serialize :: GenericsOutput :: new (& generics) ; match & input . fields { Fields :: Named (fields) => { for field in & fields . named { let field_id = serialize :: FieldId :: Struct (field . ident . clone () . unwrap ()) ; process_field (field , field_id , & cratename , & mut generics_output , & mut body) ? ; } } Fields :: Unnamed (fields) => { for (field_idx , field) in fields . unnamed . iter () . enumerate () { let field_id = serialize :: FieldId :: new_struct_unnamed (field_idx) ? ; process_field (field , field_id , & cratename , & mut generics_output , & mut body) ? ; } } Fields :: Unit => { } } generics_output . extend (& mut where_clause , & cratename) ; Ok (quote ! { # [automatically_derived] impl # impl_generics # cratename :: ser :: BorshSerialize for # name # ty_generics # where_clause { fn serialize < __W : # cratename :: io :: Write > (& self , writer : & mut __W) -> :: core :: result :: Result < () , # cratename :: io :: Error > { # body Ok (()) } } }) }
};
}
