// Generated macro for visit_struct_fields_unconditional (function)
macro_rules! Depcrate_internals_schemavisit_struct_fields_unconditional {
() => {
// Module: crate::internals::schema
// Provides: {"visit_struct_fields_unconditional"}
// Dependencies: {}
# [doc = " check param usage in fields"] fn visit_struct_fields_unconditional (fields : & Fields , visitor : & mut generics :: FindTyParams) { match & fields { Fields :: Named (fields) => { for field in & fields . named { visitor . visit_field (field) ; } } Fields :: Unnamed (fields) => { for field in & fields . unnamed { visitor . visit_field (field) ; } } Fields :: Unit => { } } }
};
}
