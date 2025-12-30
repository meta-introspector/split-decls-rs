// Generated macro for impl_95 (impl)
macro_rules! Depcrate_reader_tables_attributeimpl_95 {
() => {
// Module: crate::reader::tables::attribute
// Provides: {"impl_95"}
// Dependencies: {}
impl < 'a > Attribute < 'a > { pub fn parent (& self) -> HasAttribute < 'a > { self . decode (0) } pub fn ctor (& self) -> AttributeType < 'a > { self . decode (1) } pub fn value (& self) -> Vec < (String , Value) > { let signature = self . ctor () . signature (& []) ; debug_assert_eq ! (signature . flags , MethodCallAttributes :: HASTHIS) ; debug_assert_eq ! (signature . return_type , Type :: Void) ; let mut values = Vec :: with_capacity (signature . types . len ()) ; let mut blob = self . blob (2) ; let prolog = blob . read_u16 () ; debug_assert_eq ! (prolog , 1) ; for ty in & signature . types { let mut name = String :: new () ; let value = read_value (& mut blob , ty , & mut name) ; debug_assert ! (name . is_empty ()) ; values . push ((name , value)) ; } let named_arg_count = blob . read_u16 () ; values . reserve (named_arg_count as usize) ; for _ in 0 .. named_arg_count { let _id = blob . read_u8 () ; let ty = blob . read_type_code (& []) ; let mut name = blob . read_utf8 () ; let value = read_value (& mut blob , & ty , & mut name) ; values . push ((name , value)) ; } debug_assert_eq ! (blob . len () , 0) ; values } }
};
}
