// Generated macro for HasResolver (trait)
macro_rules! Depcrate_resolverHasResolver {
() => {
// Module: crate::resolver
// Provides: {"HasResolver"}
// Dependencies: {}
pub trait HasResolver : Copy { # [doc = " Builds a resolver for type references inside this def."] fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > ; }
};
}
