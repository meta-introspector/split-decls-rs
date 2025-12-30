// Generated macro for impl_82 (impl)
macro_rules! Depcrateimpl_82 {
() => {
// Module: crate
// Provides: {"impl_82"}
// Dependencies: {}
impl < T : FromStr > FromStr for OrderedFloat < T > { type Err = T :: Err ; # [doc = " Convert a &str to `OrderedFloat`. Returns an error if the string fails to parse."] # [doc = ""] # [doc = " ```"] # [doc = " use ordered_float::OrderedFloat;"] # [doc = ""] # [doc = " assert!(\"-10\".parse::<OrderedFloat<f32>>().is_ok());"] # [doc = " assert!(\"abc\".parse::<OrderedFloat<f32>>().is_err());"] # [doc = " assert!(\"NaN\".parse::<OrderedFloat<f32>>().is_ok());"] # [doc = " ```"] fn from_str (s : & str) -> Result < Self , Self :: Err > { T :: from_str (s) . map (OrderedFloat) } }
};
}
