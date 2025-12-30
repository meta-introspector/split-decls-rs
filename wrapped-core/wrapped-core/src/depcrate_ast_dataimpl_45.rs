// Generated macro for impl_45 (impl)
macro_rules! Depcrate_ast_dataimpl_45 {
() => {
// Module: crate::ast::data
// Provides: {"impl_45"}
// Dependencies: {}
impl Style { pub fn is_unit (self) -> bool { self == Style :: Unit } pub fn is_tuple (self) -> bool { self == Style :: Tuple } pub fn is_struct (self) -> bool { self == Style :: Struct } # [doc = " Creates a new `Fields` of the specified style with the passed-in fields."] fn with_fields < T , U : Into < Vec < T > > > (self , fields : U) -> Fields < T > { Fields :: new (self , fields . into ()) } }
};
}
