// Generated macro for test (module)
macro_rules! Depcrate_buffer_partest {
() => {
// Module: crate::buffer_par
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: { Rgb , RgbImage } ; use rayon :: iter :: { IndexedParallelIterator , ParallelIterator } ; fn test_width_height (width : u32 , height : u32 , len : usize) { let mut image = RgbImage :: new (width , height) ; assert_eq ! (image . par_enumerate_pixels_mut () . len () , len) ; assert_eq ! (image . par_enumerate_pixels () . len () , len) ; assert_eq ! (image . par_pixels_mut () . len () , len) ; assert_eq ! (image . par_pixels () . len () , len) ; } # [test] fn zero_width_zero_height () { test_width_height (0 , 0 , 0) ; } # [test] fn zero_width_nonzero_height () { test_width_height (0 , 2 , 0) ; } # [test] fn nonzero_width_zero_height () { test_width_height (2 , 0 , 0) ; } # [test] fn iter_parity () { let mut image1 = RgbImage :: from_fn (17 , 29 , | x , y | { Rgb (std :: array :: from_fn (| i | { ((x + y * 98 + i as u32 * 27) % 255) as u8 })) }) ; let mut image2 = image1 . clone () ; assert_eq ! (image1 . enumerate_pixels_mut () . collect ::< Vec < _ >> () , image2 . par_enumerate_pixels_mut () . collect ::< Vec < _ >> ()) ; assert_eq ! (image1 . enumerate_pixels () . collect ::< Vec < _ >> () , image2 . par_enumerate_pixels () . collect ::< Vec < _ >> ()) ; assert_eq ! (image1 . pixels_mut () . collect ::< Vec < _ >> () , image2 . par_pixels_mut () . collect ::< Vec < _ >> ()) ; assert_eq ! (image1 . pixels () . collect ::< Vec < _ >> () , image2 . par_pixels () . collect ::< Vec < _ >> ()) ; } }
};
}
