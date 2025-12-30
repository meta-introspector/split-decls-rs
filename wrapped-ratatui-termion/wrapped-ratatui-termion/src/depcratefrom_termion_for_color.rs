// Generated macro for from_termion_for_color (macro)
macro_rules! Depcratefrom_termion_for_color {
() => {
// Module: crate
// Provides: {"from_termion_for_color"}
// Dependencies: {}
macro_rules ! from_termion_for_color { ($ termion_color : ident , $ color : ident) => { impl FromTermion < tcolor ::$ termion_color > for Color { fn from_termion (_ : tcolor ::$ termion_color) -> Self { Color ::$ color } } impl FromTermion < tcolor :: Bg < tcolor ::$ termion_color >> for Style { fn from_termion (_ : tcolor :: Bg < tcolor ::$ termion_color >) -> Self { Style :: default () . bg (Color ::$ color) } } impl FromTermion < tcolor :: Fg < tcolor ::$ termion_color >> for Style { fn from_termion (_ : tcolor :: Fg < tcolor ::$ termion_color >) -> Self { Style :: default () . fg (Color ::$ color) } } } ; }
};
}
