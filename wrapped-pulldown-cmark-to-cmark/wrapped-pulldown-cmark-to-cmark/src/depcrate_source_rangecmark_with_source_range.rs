// Generated macro for cmark_with_source_range (function)
macro_rules! Depcrate_source_rangecmark_with_source_range {
() => {
// Module: crate::source_range
// Provides: {"cmark_with_source_range"}
// Dependencies: {}
# [doc = " As [`cmark_with_source_range_and_options`], but with default [`Options`]."] pub fn cmark_with_source_range < 'a , I , E , F > (event_and_ranges : I , source : & 'a str , mut formatter : F ,) -> Result < State < 'a > , Error > where I : Iterator < Item = (E , Option < Range < usize > >) > , E : Borrow < Event < 'a > > , F : fmt :: Write , { cmark_with_source_range_and_options (event_and_ranges , source , & mut formatter , Default :: default ()) }
};
}
