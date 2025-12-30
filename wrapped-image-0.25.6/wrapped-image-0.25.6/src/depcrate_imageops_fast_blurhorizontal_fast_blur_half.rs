// Generated macro for horizontal_fast_blur_half (function)
macro_rules! Depcrate_imageops_fast_blurhorizontal_fast_blur_half {
() => {
// Module: crate::imageops::fast_blur
// Provides: {"horizontal_fast_blur_half"}
// Dependencies: {}
fn horizontal_fast_blur_half < P : Primitive > (samples : & [P] , width : usize , height : usize , r : usize , channel_num : usize ,) -> Vec < P > { let channel_size = width * height ; let mut out_samples = vec ! [P :: from (0) . unwrap () ; channel_size * channel_num] ; let mut vals = vec ! [0.0 ; channel_num] ; let min_value = P :: DEFAULT_MIN_VALUE . to_f32 () . unwrap () ; let max_value = P :: DEFAULT_MAX_VALUE . to_f32 () . unwrap () ; for row in 0 .. height { for (channel , value) in vals . iter_mut () . enumerate () . take (channel_num) { * value = ((- (r as isize)) .. (r + 1) as isize) . map (| x | { extended_f (samples , width , height , x , row as isize , channel , channel_num ,) . to_f32 () . unwrap_or (0.0) }) . sum () ; } for column in 0 .. width { for (channel , channel_val) in vals . iter_mut () . enumerate () { let val = * channel_val / (2.0 * r as f32 + 1.0) ; let val = clamp (val , min_value , max_value) ; let val = P :: from (val) . unwrap () ; let destination_row = column ; let destination_column = row ; let destination_sample_index = channel_idx (channel , destination_column + destination_row * height , channel_num ,) ; out_samples [destination_sample_index] = val ; * channel_val = * channel_val - extended_f (samples , width , height , column as isize - r as isize , row as isize , channel , channel_num ,) . to_f32 () . unwrap_or (0.0) + extended_f (samples , width , height , { column + r + 1 } as isize , row as isize , channel , channel_num ,) . to_f32 () . unwrap_or (0.0) ; } } } out_samples }
};
}
