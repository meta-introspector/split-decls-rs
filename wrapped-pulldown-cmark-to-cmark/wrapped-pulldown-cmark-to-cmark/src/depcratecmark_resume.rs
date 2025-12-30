// Generated macro for cmark_resume (function)
macro_rules! Depcratecmark_resume {
() => {
// Module: crate
// Provides: {"cmark_resume"}
// Dependencies: {}
# [doc = " As [`cmark_resume_with_options()`], but with default [`Options`]."] pub fn cmark_resume < 'a , I , E , F > (events : I , formatter : F , state : Option < State < 'a > >) -> Result < State < 'a > , Error > where I : Iterator < Item = E > , E : Borrow < Event < 'a > > , F : fmt :: Write , { cmark_resume_with_options (events , formatter , state , Options :: default ()) }
};
}
