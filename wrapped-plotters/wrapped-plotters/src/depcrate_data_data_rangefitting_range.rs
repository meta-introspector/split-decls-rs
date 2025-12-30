// Generated macro for fitting_range (function)
macro_rules! Depcrate_data_data_rangefitting_range {
() => {
// Module: crate::data::data_range
// Provides: {"fitting_range"}
// Dependencies: {}
# [doc = " Build a range that fits the data"] # [doc = ""] # [doc = " - `iter`: the iterator over the data"] # [doc = " - **returns** The resulting range"] # [doc = ""] # [doc = " ```rust"] # [doc = " use plotters::data::fitting_range;"] # [doc = ""] # [doc = " let data = [4, 14, -2, 2, 5];"] # [doc = " let range = fitting_range(&data);"] # [doc = " assert_eq!(range, std::ops::Range { start: -2, end: 14 });"] # [doc = " ```"] pub fn fitting_range < 'a , T , I : IntoIterator < Item = & 'a T > > (iter : I) -> Range < T > where T : 'a + Zero + One + PartialOrd + Clone , { let (mut lb , mut ub) = (None , None) ; for value in iter . into_iter () { if let Some (Ordering :: Greater) = lb . as_ref () . map_or (Some (Ordering :: Greater) , | lbv : & T | lbv . partial_cmp (value)) { lb = Some (value . clone ()) ; } if let Some (Ordering :: Less) = ub . as_ref () . map_or (Some (Ordering :: Less) , | ubv : & T | ubv . partial_cmp (value)) { ub = Some (value . clone ()) ; } } lb . unwrap_or_else (Zero :: zero) .. ub . unwrap_or_else (One :: one) }
};
}
