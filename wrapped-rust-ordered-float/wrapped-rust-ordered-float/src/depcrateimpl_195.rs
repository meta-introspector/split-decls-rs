// Generated macro for impl_195 (impl)
macro_rules! Depcrateimpl_195 {
() => {
// Module: crate
// Provides: {"impl_195"}
// Dependencies: {}
impl < T : FloatCore + FromStr > FromStr for NotNan < T > { type Err = ParseNotNanError < T :: Err > ; # [doc = " Convert a &str to `NotNan`. Returns an error if the string fails to parse,"] # [doc = " or if the resulting value is NaN"] # [doc = ""] # [doc = " ```"] # [doc = " use ordered_float::NotNan;"] # [doc = ""] # [doc = " assert!(\"-10\".parse::<NotNan<f32>>().is_ok());"] # [doc = " assert!(\"abc\".parse::<NotNan<f32>>().is_err());"] # [doc = " assert!(\"NaN\".parse::<NotNan<f32>>().is_err());"] # [doc = " ```"] fn from_str (src : & str) -> Result < Self , Self :: Err > { src . parse () . map_err (ParseNotNanError :: ParseFloatError) . and_then (| f | NotNan :: new (f) . map_err (| _ | ParseNotNanError :: IsNaN)) } }
};
}
