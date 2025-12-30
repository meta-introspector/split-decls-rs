// Generated macro for impl_148 (impl)
macro_rules! Depcrate_contextimpl_148 {
() => {
// Module: crate::context
// Provides: {"impl_148"}
// Dependencies: {}
impl TypeLocation { pub (crate) fn complete_lifetimes (& self) -> bool { matches ! (self , TypeLocation :: GenericArg { corresponding_param : Some (ast :: GenericParam :: LifetimeParam (_)) , .. }) } pub (crate) fn complete_consts (& self) -> bool { matches ! (self , TypeLocation :: GenericArg { corresponding_param : Some (ast :: GenericParam :: ConstParam (_)) , .. } | TypeLocation :: AssocConstEq) } pub (crate) fn complete_types (& self) -> bool { match self { TypeLocation :: GenericArg { corresponding_param : Some (param) , .. } => { matches ! (param , ast :: GenericParam :: TypeParam (_)) } TypeLocation :: AssocConstEq => false , TypeLocation :: AssocTypeEq => true , TypeLocation :: ImplTrait => false , _ => true , } } pub (crate) fn complete_self_type (& self) -> bool { self . complete_types () && ! matches ! (self , TypeLocation :: ImplTarget | TypeLocation :: ImplTrait) } }
};
}
