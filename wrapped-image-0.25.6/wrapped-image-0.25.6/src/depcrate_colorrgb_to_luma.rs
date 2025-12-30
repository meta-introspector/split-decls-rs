// Generated macro for rgb_to_luma (function)
macro_rules! Depcrate_colorrgb_to_luma {
() => {
// Module: crate::color
// Provides: {"rgb_to_luma"}
// Dependencies: {}
# [inline] fn rgb_to_luma < T : Primitive + Enlargeable > (rgb : & [T]) -> T { let l = < T :: Larger as NumCast > :: from (SRGB_LUMA [0]) . unwrap () * rgb [0] . to_larger () + < T :: Larger as NumCast > :: from (SRGB_LUMA [1]) . unwrap () * rgb [1] . to_larger () + < T :: Larger as NumCast > :: from (SRGB_LUMA [2]) . unwrap () * rgb [2] . to_larger () ; T :: clamp_from (l / < T :: Larger as NumCast > :: from (SRGB_LUMA_DIV) . unwrap ()) }
};
}
