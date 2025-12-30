// Generated macro for impl_5 (impl)
macro_rules! Depcrate_paletteimpl_5 {
() => {
// Module: crate::palette
// Provides: {"impl_5"}
// Dependencies: {}
impl Palette { # [doc = " Look up the [`anstyle::RgbColor`] in the palette"] pub const fn get (& self , color : anstyle :: AnsiColor) -> Rgb { let color = anstyle :: Ansi256Color :: from_ansi (color) ; * self . get_ansi256_ref (color) } const fn get_ansi256_ref (& self , color : anstyle :: Ansi256Color) -> & Rgb { let index = color . index () as usize ; & self . 0 [index] } pub (crate) const fn rgb_from_ansi (& self , color : anstyle :: AnsiColor) -> anstyle :: RgbColor { self . get (color) } pub (crate) const fn rgb_from_index (& self , index : u8) -> Option < anstyle :: RgbColor > { let index = index as usize ; if index < self . 0 . len () { Some (self . 0 [index]) } else { None } } pub (crate) const fn find_match (& self , color : anstyle :: RgbColor) -> anstyle :: AnsiColor { let mut best_index = 0 ; let mut best_distance = crate :: distance (color , self . 0 [best_index]) ; let mut index = best_index + 1 ; while index < self . 0 . len () { let distance = crate :: distance (color , self . 0 [index]) ; if distance < best_distance { best_index = index ; best_distance = distance ; } index += 1 ; } if let Some (color) = anstyle :: Ansi256Color (best_index as u8) . into_ansi () { color } else { # [allow (clippy :: no_effect)] ["best_index is out of bounds"] [best_index] ; anstyle :: AnsiColor :: Black } } }
};
}
