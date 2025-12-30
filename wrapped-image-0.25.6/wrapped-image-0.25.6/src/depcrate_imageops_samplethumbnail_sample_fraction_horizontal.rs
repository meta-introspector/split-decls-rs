// Generated macro for thumbnail_sample_fraction_horizontal (function)
macro_rules! Depcrate_imageops_samplethumbnail_sample_fraction_horizontal {
() => {
// Module: crate::imageops::sample
// Provides: {"thumbnail_sample_fraction_horizontal"}
// Dependencies: {}
# [doc = " Get a thumbnail pixel where the input window encloses at least a vertical pixel."] fn thumbnail_sample_fraction_horizontal < I , P , S > (image : & I , left : u32 , fraction_horizontal : f32 , bottom : u32 , top : u32 ,) -> (S , S , S , S) where I : GenericImageView < Pixel = P > , P : Pixel < Subpixel = S > , S : Primitive + Enlargeable , { let fract = fraction_horizontal ; let mut sum_left = ThumbnailSum :: zeroed () ; let mut sum_right = ThumbnailSum :: zeroed () ; for x in bottom .. top { let k_left = image . get_pixel (left , x) ; sum_left . add_pixel (k_left) ; let k_right = image . get_pixel (left + 1 , x) ; sum_right . add_pixel (k_right) ; } let fact_right = fract / ((top - bottom) as f32) ; let fact_left = (1. - fract) / ((top - bottom) as f32) ; let mix_left_and_right = | leftv : S :: Larger , rightv : S :: Larger | { < S as NumCast > :: from (fact_left * leftv . to_f32 () . unwrap () + fact_right * rightv . to_f32 () . unwrap () ,) . expect ("Average sample value should fit into sample type") } ; (mix_left_and_right (sum_left . 0 , sum_right . 0) , mix_left_and_right (sum_left . 1 , sum_right . 1) , mix_left_and_right (sum_left . 2 , sum_right . 2) , mix_left_and_right (sum_left . 3 , sum_right . 3) ,) }
};
}
