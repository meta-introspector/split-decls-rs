// Generated macro for thumbnail_sample_fraction_both (function)
macro_rules! Depcrate_imageops_samplethumbnail_sample_fraction_both {
() => {
// Module: crate::imageops::sample
// Provides: {"thumbnail_sample_fraction_both"}
// Dependencies: {}
# [doc = " Get a single pixel for a thumbnail where the input window does not enclose any full pixel."] fn thumbnail_sample_fraction_both < I , P , S > (image : & I , left : u32 , fraction_vertical : f32 , bottom : u32 , fraction_horizontal : f32 ,) -> (S , S , S , S) where I : GenericImageView < Pixel = P > , P : Pixel < Subpixel = S > , S : Primitive + Enlargeable , { # [allow (deprecated)] let k_bl = image . get_pixel (left , bottom) . channels4 () ; # [allow (deprecated)] let k_tl = image . get_pixel (left , bottom + 1) . channels4 () ; # [allow (deprecated)] let k_br = image . get_pixel (left + 1 , bottom) . channels4 () ; # [allow (deprecated)] let k_tr = image . get_pixel (left + 1 , bottom + 1) . channels4 () ; let frac_v = fraction_vertical ; let frac_h = fraction_horizontal ; let fact_tr = frac_v * frac_h ; let fact_tl = frac_v * (1. - frac_h) ; let fact_br = (1. - frac_v) * frac_h ; let fact_bl = (1. - frac_v) * (1. - frac_h) ; let mix = | br : S , tr : S , bl : S , tl : S | { < S as NumCast > :: from (fact_br * br . to_f32 () . unwrap () + fact_tr * tr . to_f32 () . unwrap () + fact_bl * bl . to_f32 () . unwrap () + fact_tl * tl . to_f32 () . unwrap () ,) . expect ("Average sample value should fit into sample type") } ; (mix (k_br . 0 , k_tr . 0 , k_bl . 0 , k_tl . 0) , mix (k_br . 1 , k_tr . 1 , k_bl . 1 , k_tl . 1) , mix (k_br . 2 , k_tr . 2 , k_bl . 2 , k_tl . 2) , mix (k_br . 3 , k_tr . 3 , k_bl . 3 , k_tl . 3) ,) }
};
}
