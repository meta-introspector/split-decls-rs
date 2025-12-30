// Generated macro for impl_126 (impl)
macro_rules! Depcrate_coord_ranged1d_combinators_ckpsimpl_126 {
() => {
// Module: crate::coord::ranged1d::combinators::ckps
// Provides: {"impl_126"}
// Dependencies: {}
impl < I : Ranged > WithKeyPoints < I > { # [doc = " Specify the light key points, which is used to render the light mesh line"] pub fn with_light_points < T : IntoIterator < Item = I :: ValueType > > (mut self , iter : T) -> Self { self . light_points . clear () ; self . light_points . extend (iter) ; self } # [doc = " Get a reference to the bold points"] pub fn bold_points (& self) -> & [I :: ValueType] { self . bold_points . as_ref () } # [doc = " Get a mut reference to the bold points"] pub fn bold_points_mut (& mut self) -> & mut [I :: ValueType] { self . bold_points . as_mut () } # [doc = " Get a reference to light key points"] pub fn light_points (& self) -> & [I :: ValueType] { self . light_points . as_ref () } # [doc = " Get a mut reference to the light key points"] pub fn light_points_mut (& mut self) -> & mut [I :: ValueType] { self . light_points . as_mut () } }
};
}
