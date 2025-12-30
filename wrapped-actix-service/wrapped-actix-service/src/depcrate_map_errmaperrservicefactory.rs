// Generated macro for MapErrServiceFactory (struct)
macro_rules! Depcrate_map_errMapErrServiceFactory {
() => {
// Module: crate::map_err
// Provides: {"MapErrServiceFactory"}
// Dependencies: {}
# [doc = " Factory for the `map_err` combinator, changing the type of a new"] # [doc = " service's error."] # [doc = ""] # [doc = " This is created by the `NewServiceExt::map_err` method."] pub struct MapErrServiceFactory < SF , Req , F , E > where SF : ServiceFactory < Req > , F : Fn (SF :: Error) -> E + Clone , { a : SF , f : F , e : PhantomData < fn (Req) -> E > , }
};
}
