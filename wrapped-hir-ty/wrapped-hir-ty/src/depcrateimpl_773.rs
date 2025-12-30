// Generated macro for impl_773 (impl)
macro_rules! Depcrateimpl_773 {
() => {
// Module: crate
// Provides: {"impl_773"}
// Dependencies: {}
impl < 'db > rustc_type_ir :: TypeVisitor < DbInterner < 'db > > for ParamCollector { type Result = () ; fn visit_ty (& mut self , ty : Ty < 'db >) -> Self :: Result { if let TyKind :: Param (param) = ty . kind () { self . params . insert (param . id . into ()) ; } ty . super_visit_with (self) ; } fn visit_const (& mut self , konst : Const < 'db >) -> Self :: Result { if let ConstKind :: Param (param) = konst . kind () { self . params . insert (param . id . into ()) ; } konst . super_visit_with (self) ; } }
};
}
