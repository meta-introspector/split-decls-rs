// Generated macro for impl_7 (impl)
macro_rules! Depcrate_paletteimpl_7 {
() => {
// Module: crate::palette
// Provides: {"impl_7"}
// Dependencies: {}
impl std :: ops :: Index < anstyle :: AnsiColor > for Palette { type Output = Rgb ; # [inline] fn index (& self , color : anstyle :: AnsiColor) -> & Rgb { let color = anstyle :: Ansi256Color :: from_ansi (color) ; self . get_ansi256_ref (color) } }
};
}
