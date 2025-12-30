// Generated macro for impl_240 (impl)
macro_rules! Depcrate_gradientimpl_240 {
() => {
// Module: crate::gradient
// Provides: {"impl_240"}
// Dependencies: {}
impl CGGradient { pub fn create_with_color_components (color_space : & CGColorSpace , components : & [CGFloat] , locations : & [CGFloat] , count : usize ,) -> CGGradient { unsafe { let result = CGGradientCreateWithColorComponents (color_space . as_ptr () , components . as_ptr () , locations . as_ptr () , count ,) ; assert ! (! result . is_null ()) ; Self :: from_ptr (result) } } pub fn create_with_colors (color_space : & CGColorSpace , colors : & CFArray < CGColor > , locations : & [CGFloat] ,) -> CGGradient { unsafe { let result = CGGradientCreateWithColors (color_space . as_ptr () , colors . as_concrete_TypeRef () , locations . as_ptr () ,) ; assert ! (! result . is_null ()) ; Self :: from_ptr (result) } } }
};
}
