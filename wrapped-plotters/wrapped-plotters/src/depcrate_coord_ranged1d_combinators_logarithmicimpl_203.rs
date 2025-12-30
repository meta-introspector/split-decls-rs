// Generated macro for impl_203 (impl)
macro_rules! Depcrate_coord_ranged1d_combinators_logarithmicimpl_203 {
() => {
// Module: crate::coord::ranged1d::combinators::logarithmic
// Provides: {"impl_203"}
// Dependencies: {}
impl < V : LogScalable > Ranged for LogCoord < V > { type FormatOption = DefaultFormatting ; type ValueType = V ; fn map (& self , value : & V , limit : (i32 , i32)) -> i32 { let fv = self . value_to_f64 (value) ; let value_ln = fv . ln () ; self . linear . map (& value_ln , limit) } fn key_points < Hint : KeyPointHint > (& self , hint : Hint) -> Vec < Self :: ValueType > { let max_points = hint . max_num_points () ; let base = self . base ; let base_ln = base . ln () ; let Range { mut start , mut end } = self . normalized ; if start > end { std :: mem :: swap (& mut start , & mut end) ; } let bold_count = ((end / start) . ln () . abs () / base_ln) . floor () . max (1.0) as usize ; let light_density = if max_points < bold_count { 0 } else { let density = 1 + (max_points - bold_count) / bold_count ; let mut exp = 1 ; while exp * 10 <= density { exp *= 10 ; } exp - 1 } ; let mut multiplier = base ; let mut cnt = 1 ; while max_points < bold_count / cnt { multiplier *= base ; cnt += 1 ; } let mut ret = vec ! [] ; let mut val = (base) . powf ((start . ln () / base_ln) . ceil ()) ; while val <= end { if ! self . is_inf (val) { ret . push (self . f64_to_value (val)) ; } for i in 1 ..= light_density { let v = val * (1.0 + multiplier / f64 :: from (light_density as u32 + 1) * f64 :: from (i as u32)) ; if v > end { break ; } if ! self . is_inf (val) { ret . push (self . f64_to_value (v)) ; } } val *= multiplier ; } ret } fn range (& self) -> Range < V > { self . logic . clone () } }
};
}
