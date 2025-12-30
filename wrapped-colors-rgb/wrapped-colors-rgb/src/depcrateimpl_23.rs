// Generated macro for impl_23 (impl)
macro_rules! Depcrateimpl_23 {
() => {
// Module: crate
// Provides: {"impl_23"}
// Dependencies: {}
impl ColorsWidget { # [doc = " Setup the colors to render."] # [doc = ""] # [doc = " This is called once per frame to setup the colors to render. It caches the colors so that"] # [doc = " they don't need to be recalculated every frame."] # [expect (clippy :: cast_precision_loss)] fn setup_colors (& mut self , size : Rect) { let Rect { width , height , .. } = size ; let height = height as usize * 2 ; let width = width as usize ; if self . colors . len () == height && self . colors [0] . len () == width { return ; } self . colors = Vec :: with_capacity (height) ; for y in 0 .. height { let mut row = Vec :: with_capacity (width) ; for x in 0 .. width { let hue = x as f32 * 360.0 / width as f32 ; let value = (height - y) as f32 / height as f32 ; let saturation = Okhsv :: max_saturation () ; let color = Okhsv :: new (hue , saturation , value) ; let color = Srgb :: < f32 > :: from_color_unclamped (color) ; let color : Srgb < u8 > = color . into_format () ; let color = Color :: Rgb (color . red , color . green , color . blue) ; row . push (color) ; } self . colors . push (row) ; } } }
};
}
