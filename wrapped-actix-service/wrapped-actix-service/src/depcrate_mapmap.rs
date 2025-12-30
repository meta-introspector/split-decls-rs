// Generated macro for Map (struct)
macro_rules! Depcrate_mapMap {
() => {
// Module: crate::map
// Provides: {"Map"}
// Dependencies: {}
# [doc = " Service for the `map` combinator, changing the type of a service's response."] # [doc = ""] # [doc = " This is created by the `ServiceExt::map` method."] pub struct Map < A , F , Req , Res > { service : A , f : F , _t : PhantomData < fn (Req) -> Res > , }
};
}
