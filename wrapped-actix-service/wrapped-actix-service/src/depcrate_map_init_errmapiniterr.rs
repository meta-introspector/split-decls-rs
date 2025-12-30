// Generated macro for MapInitErr (struct)
macro_rules! Depcrate_map_init_errMapInitErr {
() => {
// Module: crate::map_init_err
// Provides: {"MapInitErr"}
// Dependencies: {}
# [doc = " `MapInitErr` service combinator"] pub struct MapInitErr < A , F , Req , Err > { a : A , f : F , e : PhantomData < fn (Req) -> Err > , }
};
}
