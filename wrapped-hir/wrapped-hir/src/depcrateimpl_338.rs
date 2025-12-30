// Generated macro for impl_338 (impl)
macro_rules! Depcrateimpl_338 {
() => {
// Module: crate
// Provides: {"impl_338"}
// Dependencies: {}
impl TupleField { pub fn name (& self) -> Name { Name :: new_tuple_field (self . index as usize) } pub fn ty < 'db > (& self , db : & 'db dyn HirDatabase) -> Type < 'db > { let ty = db . infer (self . owner) . tuple_field_access_type (self . tuple) . as_slice (Interner) . get (self . index as usize) . and_then (| arg | arg . ty (Interner)) . cloned () . unwrap_or_else (| | TyKind :: Error . intern (Interner)) ; Type { env : db . trait_environment_for_body (self . owner) , ty , _pd : PhantomCovariantLifetime :: new () , } } }
};
}
