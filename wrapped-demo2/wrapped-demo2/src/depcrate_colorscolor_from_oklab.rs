// Generated macro for color_from_oklab (function)
macro_rules! Depcrate_colorscolor_from_oklab {
() => {
// Module: crate::colors
// Provides: {"color_from_oklab"}
// Dependencies: {}
# [doc = " Convert a hue and value into an RGB color via the Oklab color space."] # [doc = ""] # [doc = " See <https://bottosson.github.io/posts/oklab/> for more details."] pub fn color_from_oklab (hue : f32 , saturation : f32 , value : f32) -> Color { let color : Srgb = Okhsv :: new (hue , saturation , value) . into_color () ; let color = color . into_format () ; Color :: Rgb (color . red , color . green , color . blue) }
};
}
