// Generated macro for from_lt_id (function)
macro_rules! Depcrate_genericsfrom_lt_id {
() => {
// Module: crate::generics
// Provides: {"from_lt_id"}
// Dependencies: {}
fn from_lt_id < 'a > (it : & 'a Generics ,) -> impl Fn ((LocalLifetimeParamId , & 'a LifetimeParamData)) -> (GenericParamId , GenericParamDataRef < 'a >) { move | (local_id , p) : (_ , _) | { (GenericParamId :: LifetimeParamId (LifetimeParamId { parent : it . def , local_id }) , GenericParamDataRef :: LifetimeParamData (p) ,) } }
};
}
