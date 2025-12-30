// Generated macro for visit_struct_fields (function)
macro_rules! Depcrate_internals_schemavisit_struct_fields {
() => {
// Module: crate::internals::schema
// Provides: {"visit_struct_fields"}
// Dependencies: {}
# [doc = " check param usage in fields with respect to `borsh(skip)` attribute usage"] fn visit_struct_fields (fields : & Fields , visitor : & mut generics :: FindTyParams) -> syn :: Result < () > { match & fields { Fields :: Named (fields) => { for field in & fields . named { visit_field (field , visitor) ? ; } } Fields :: Unnamed (fields) => { for field in & fields . unnamed { visit_field (field , visitor) ? ; } } Fields :: Unit => { } } Ok (()) }
};
}
