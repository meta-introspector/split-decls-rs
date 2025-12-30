// Generated macro for generate_yearly_keypoints (function)
macro_rules! Depcrate_coord_ranged1d_types_datetimegenerate_yearly_keypoints {
() => {
// Module: crate::coord::ranged1d::types::datetime
// Provides: {"generate_yearly_keypoints"}
// Dependencies: {}
fn generate_yearly_keypoints < T : TimeValue > (max_points : usize , mut start_year : i32 , start_month : u32 , mut end_year : i32 , end_month : u32 , builder : & T ,) -> Vec < T > { if start_month > end_month { end_year -= 1 ; } let mut exp10 = 1 ; while (end_year - start_year + 1) as usize / (exp10 * 10) > max_points { exp10 *= 10 ; } let mut freq = exp10 ; for try_freq in & [1 , 2 , 5 , 10] { freq = * try_freq * exp10 ; if (end_year - start_year + 1) as usize / (exp10 * * try_freq) <= max_points { break ; } } let mut ret = vec ! [] ; while start_year <= end_year { ret . push (T :: earliest_after_date (builder . ymd (start_year , start_month , 1 ,))) ; start_year += freq as i32 ; } ret }
};
}
