// Generated macro for impl_740 (impl)
macro_rules! Depcrate_usage_lifetimesimpl_740 {
() => {
// Module: crate::usage::lifetimes
// Provides: {"impl_740"}
// Dependencies: {}
impl UsesLifetimes for syn :: GenericArgument { fn uses_lifetimes < 'a > (& self , options : & Options , lifetimes : & 'a LifetimeSet ,) -> LifetimeRefSet < 'a > { match * self { syn :: GenericArgument :: Type (ref v) => v . uses_lifetimes (options , lifetimes) , syn :: GenericArgument :: AssocType (ref v) => v . uses_lifetimes (options , lifetimes) , syn :: GenericArgument :: Lifetime (ref v) => v . uses_lifetimes (options , lifetimes) , syn :: GenericArgument :: Constraint (ref v) => v . uses_lifetimes (options , lifetimes) , syn :: GenericArgument :: AssocConst (_) | syn :: GenericArgument :: Const (_) => { Default :: default () } _ => panic ! ("Unknown syn::GenericArgument: {:?}" , self) , } } }
};
}
