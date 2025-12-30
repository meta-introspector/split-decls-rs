// Generated macro for thumbnail_sample_fraction_vertical (function)
macro_rules! Depcrate_imageops_samplethumbnail_sample_fraction_vertical {
() => {
// Module: crate::imageops::sample
// Provides: {"thumbnail_sample_fraction_vertical"}
// Dependencies: {}
# [doc = " Get a thumbnail pixel where the input window encloses at least a horizontal pixel."] fn thumbnail_sample_fraction_vertical < I , P , S > (image : & I , left : u32 , right : u32 , bottom : u32 , fraction_vertical : f32 ,) -> (S , S , S , S) where I : GenericImageView < Pixel = P > , P : Pixel < Subpixel = S > , S : Primitive + Enlargeable , { let fract = fraction_vertical ; let mut sum_bot = ThumbnailSum :: zeroed () ; let mut sum_top = ThumbnailSum :: zeroed () ; for x in left .. right { let k_bot = image . get_pixel (x , bottom) ; sum_bot . add_pixel (k_bot) ; let k_top = image . get_pixel (x , bottom + 1) ; sum_top . add_pixel (k_top) ; } let fact_top = fract / ((right - left) as f32) ; let fact_bot = (1. - fract) / ((right - left) as f32) ; let mix_bot_and_top = | botv : S :: Larger , topv : S :: Larger | { < S as NumCast > :: from (fact_bot * botv . to_f32 () . unwrap () + fact_top * topv . to_f32 () . unwrap ()) . expect ("Average sample value should fit into sample type") } ; (mix_bot_and_top (sum_bot . 0 , sum_top . 0) , mix_bot_and_top (sum_bot . 1 , sum_top . 1) , mix_bot_and_top (sum_bot . 2 , sum_top . 2) , mix_bot_and_top (sum_bot . 3 , sum_top . 3) ,) }
};
}
