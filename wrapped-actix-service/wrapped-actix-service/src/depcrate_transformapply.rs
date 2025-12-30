// Generated macro for apply (function)
macro_rules! Depcrate_transformapply {
() => {
// Module: crate::transform
// Provides: {"apply"}
// Dependencies: {}
# [doc = " Apply a [`Transform`] to a [`Service`]."] pub fn apply < T , S , I , Req > (t : T , factory : I) -> ApplyTransform < T , S , Req > where I : IntoServiceFactory < S , Req > , S : ServiceFactory < Req > , T : Transform < S :: Service , Req , InitError = S :: InitError > , { ApplyTransform :: new (t , factory . into_factory ()) }
};
}
