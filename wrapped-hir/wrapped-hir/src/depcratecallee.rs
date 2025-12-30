// Generated macro for Callee (enum)
macro_rules! DepcrateCallee {
() => {
// Module: crate
// Provides: {"Callee"}
// Dependencies: {}
# [derive (Clone , PartialEq , Eq , Hash , Debug)] enum Callee < 'db > { Def (CallableDefId) , Closure (InternedClosureId , GenericArgs < 'db >) , CoroutineClosure (InternedCoroutineId , GenericArgs < 'db >) , FnPtr , FnImpl (FnTrait) , }
};
}
