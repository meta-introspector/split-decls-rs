// Generated macro for cmark (function)
macro_rules! Depcratecmark {
() => {
// Module: crate
// Provides: {"cmark"}
// Dependencies: {}
# [doc = " As [`cmark_with_options()`], but with default [`Options`]."] pub fn cmark < 'a , I , E , F > (events : I , mut formatter : F) -> Result < State < 'a > , Error > where I : Iterator < Item = E > , E : Borrow < Event < 'a > > , F : fmt :: Write , { cmark_with_options (events , & mut formatter , Default :: default ()) }
};
}
