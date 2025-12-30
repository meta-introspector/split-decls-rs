// Generated macro for impl_21 (impl)
macro_rules! Depcrate_colorimpl_21 {
() => {
// Module: crate::color
// Provides: {"impl_21"}
// Dependencies: {}
impl Display for Color { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let mut write_space = None ; if let Some (fg) = self . foreground { fg . fmt (f) ? ; write_space = Some (()) ; } if let Some (bg) = self . background { if write_space . take () . is_some () { write ! (f , " ") ? ; } bg . fmt (f) ? ; write_space = Some (()) ; } if ! self . attributes . is_empty () { if write_space . take () . is_some () { write ! (f , " ") ? ; } self . attributes . fmt (f) ? ; } Ok (()) } }
};
}
