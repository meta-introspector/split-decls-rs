// Generated macro for TransformMapInitErr (struct)
macro_rules! Depcrate_transform_errTransformMapInitErr {
() => {
// Module: crate::transform_err
// Provides: {"TransformMapInitErr"}
// Dependencies: {}
# [doc = " Transform for the [`TransformExt::map_init_err`] combinator, changing the type of a new"] # [doc = " [`Transform`]'s initialization error."] pub struct TransformMapInitErr < T , S , Req , F , E > { transform : T , mapper : F , _phantom : PhantomData < fn (Req) -> (S , E) > , }
};
}
