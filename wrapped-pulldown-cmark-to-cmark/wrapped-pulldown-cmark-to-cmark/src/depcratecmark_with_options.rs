// Generated macro for cmark_with_options (function)
macro_rules! Depcratecmark_with_options {
() => {
// Module: crate
// Provides: {"cmark_with_options"}
// Dependencies: {}
# [doc = " As [`cmark_resume_with_options()`], but with the [`State`] finalized."] pub fn cmark_with_options < 'a , I , E , F > (events : I , mut formatter : F , options : Options < '_ >) -> Result < State < 'a > , Error > where I : Iterator < Item = E > , E : Borrow < Event < 'a > > , F : fmt :: Write , { let state = cmark_resume_with_options (events , & mut formatter , Default :: default () , options) ? ; state . finalize (formatter) }
};
}
