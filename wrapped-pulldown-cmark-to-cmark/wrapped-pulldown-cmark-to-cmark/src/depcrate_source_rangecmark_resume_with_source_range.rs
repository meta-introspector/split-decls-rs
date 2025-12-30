// Generated macro for cmark_resume_with_source_range (function)
macro_rules! Depcrate_source_rangecmark_resume_with_source_range {
() => {
// Module: crate::source_range
// Provides: {"cmark_resume_with_source_range"}
// Dependencies: {}
# [doc = " As [`cmark_resume_with_source_range_and_options`], but with default [`Options`]."] pub fn cmark_resume_with_source_range < 'a , I , E , F > (event_and_ranges : I , source : & 'a str , formatter : F , state : Option < State < 'a > > ,) -> Result < State < 'a > , Error > where I : Iterator < Item = (E , Option < Range < usize > >) > , E : Borrow < Event < 'a > > , F : fmt :: Write , { cmark_resume_with_source_range_and_options (event_and_ranges , source , formatter , state , Options :: default ()) }
};
}
