// Generated macro for MapErr (struct)
macro_rules! Depcrate_map_errMapErr {
() => {
// Module: crate::map_err
// Provides: {"MapErr"}
// Dependencies: {}
# [doc = " Service for the `map_err` combinator, changing the type of a service's error."] # [doc = ""] # [doc = " This is created by the `ServiceExt::map_err` method."] pub struct MapErr < S , Req , F , E > { service : S , mapper : F , _t : PhantomData < fn (Req) -> E > , }
};
}
