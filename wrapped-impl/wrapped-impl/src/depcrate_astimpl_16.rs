// Generated macro for impl_16 (impl)
macro_rules! Depcrate_astimpl_16 {
() => {
// Module: crate::ast
// Provides: {"impl_16"}
// Dependencies: {}
impl < 'a > Struct < 'a > { fn from_syn (node : & 'a DeriveInput , data : & 'a DataStruct) -> Result < Self > { let mut attrs = attr :: get (& node . attrs) ? ; let scope = ParamsInScope :: new (& node . generics) ; let fields = Field :: multiple_from_syn (& data . fields , & scope) ? ; if let Some (display) = & mut attrs . display { let container = ContainerKind :: from_struct (data) ; display . expand_shorthand (& fields , container) ? ; } Ok (Struct { attrs , ident : node . ident . clone () , generics : & node . generics , fields , }) } }
};
}
