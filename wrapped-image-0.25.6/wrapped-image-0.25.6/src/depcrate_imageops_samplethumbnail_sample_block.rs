// Generated macro for thumbnail_sample_block (function)
macro_rules! Depcrate_imageops_samplethumbnail_sample_block {
() => {
// Module: crate::imageops::sample
// Provides: {"thumbnail_sample_block"}
// Dependencies: {}
# [doc = " Get a pixel for a thumbnail where the input window encloses at least a full pixel."] fn thumbnail_sample_block < I , P , S > (image : & I , left : u32 , right : u32 , bottom : u32 , top : u32 ,) -> (S , S , S , S) where I : GenericImageView < Pixel = P > , P : Pixel < Subpixel = S > , S : Primitive + Enlargeable , { let mut sum = ThumbnailSum :: zeroed () ; for y in bottom .. top { for x in left .. right { let k = image . get_pixel (x , y) ; sum . add_pixel (k) ; } } let n = < S :: Larger as NumCast > :: from ((right - left) * (top - bottom)) . unwrap () ; let round = < S :: Larger as NumCast > :: from (n / NumCast :: from (2) . unwrap ()) . unwrap () ; (S :: clamp_from ((sum . 0 + round) / n) , S :: clamp_from ((sum . 1 + round) / n) , S :: clamp_from ((sum . 2 + round) / n) , S :: clamp_from ((sum . 3 + round) / n) ,) }
};
}
