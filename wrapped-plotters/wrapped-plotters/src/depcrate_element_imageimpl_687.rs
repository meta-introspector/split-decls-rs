// Generated macro for impl_687 (impl)
macro_rules! Depcrate_element_imageimpl_687 {
() => {
// Module: crate::element::image
// Provides: {"impl_687"}
// Dependencies: {}
# [cfg (all (not (all (target_arch = "wasm32" , not (target_os = "wasi"))) , feature = "image"))] impl < 'a , Coord > From < (Coord , DynamicImage) > for BitMapElement < 'a , Coord , BGRXPixel > { fn from ((pos , image) : (Coord , DynamicImage)) -> Self { let (w , h) = image . dimensions () ; let rgb_image = image . to_rgb8 () . into_raw () ; Self { pos , image : Buffer :: Owned (rgb_image) , size : (w , h) , phantom : PhantomData , } } }
};
}
