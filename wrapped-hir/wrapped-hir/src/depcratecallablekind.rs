// Generated macro for CallableKind (enum)
macro_rules! DepcrateCallableKind {
() => {
// Module: crate
// Provides: {"CallableKind"}
// Dependencies: {}
pub enum CallableKind < 'db > { Function (Function) , TupleStruct (Struct) , TupleEnumVariant (Variant) , Closure (Closure < 'db >) , FnPtr , FnImpl (FnTrait) , }
};
}
