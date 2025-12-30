// Generated macro for color_for_constraint (function)
macro_rules! Depcratecolor_for_constraint {
() => {
// Module: crate
// Provides: {"color_for_constraint"}
// Dependencies: {}
const fn color_for_constraint (constraint : Constraint) -> Color { use tailwind :: { BLUE , SLATE } ; match constraint { Constraint :: Min (_) => BLUE . c900 , Constraint :: Max (_) => BLUE . c800 , Constraint :: Length (_) => SLATE . c700 , Constraint :: Percentage (_) => SLATE . c800 , Constraint :: Ratio (_ , _) => SLATE . c900 , Constraint :: Fill (_) => SLATE . c950 , } }
};
}
