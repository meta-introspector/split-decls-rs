// Generated macro for cmark_with_source_range_and_options (function)
macro_rules! Depcrate_source_rangecmark_with_source_range_and_options {
() => {
// Module: crate::source_range
// Provides: {"cmark_with_source_range_and_options"}
// Dependencies: {}
# [doc = " As [`cmark_resume_with_source_range_and_options`], but with the [`State`] finalized."] pub fn cmark_with_source_range_and_options < 'a , I , E , F > (event_and_ranges : I , source : & 'a str , mut formatter : F , options : Options < '_ > ,) -> Result < State < 'a > , Error > where I : Iterator < Item = (E , Option < Range < usize > >) > , E : Borrow < Event < 'a > > , F : fmt :: Write , { let state = cmark_resume_with_source_range_and_options (event_and_ranges , source , & mut formatter , Default :: default () , options ,) ? ; state . finalize (formatter) }
};
}
