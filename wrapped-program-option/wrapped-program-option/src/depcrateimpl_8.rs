// Generated macro for impl_8 (impl)
macro_rules! Depcrateimpl_8 {
() => {
// Module: crate
// Provides: {"impl_8"}
// Dependencies: {}
impl < T : Default > COption < T > { # [doc = " Returns the contained value or a default"] # [doc = ""] # [doc = " Consumes the `self` argument then, if [`COption::Some`], returns the contained"] # [doc = " value, otherwise if [`COption::None`], returns the [default value] for that"] # [doc = " type."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Converts a string to an integer, turning poorly-formed strings"] # [doc = " into 0 (the default value for integers). [`parse`] converts"] # [doc = " a string to any other type that implements [`FromStr`], returning"] # [doc = " [`COption::None`] on error."] # [doc = ""] # [doc = " ```ignore"] # [doc = " let good_year_from_input = \"1909\";"] # [doc = " let bad_year_from_input = \"190blarg\";"] # [doc = " let good_year = good_year_from_input.parse().ok().unwrap_or_default();"] # [doc = " let bad_year = bad_year_from_input.parse().ok().unwrap_or_default();"] # [doc = ""] # [doc = " assert_eq!(1909, good_year);"] # [doc = " assert_eq!(0, bad_year);"] # [doc = " ```"] # [doc = ""] # [doc = " [`COption::Some`]: #variant.COption::Some"] # [doc = " [`COption::None`]: #variant.COption::None"] # [doc = " [default value]: ../default/trait.Default.html#tymethod.default"] # [doc = " [`parse`]: ../../std/primitive.str.html#method.parse"] # [doc = " [`FromStr`]: ../../std/str/trait.FromStr.html"] # [inline] pub fn unwrap_or_default (self) -> T { match self { COption :: Some (x) => x , COption :: None => T :: default () , } } }
};
}
