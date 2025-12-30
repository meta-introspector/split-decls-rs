// Generated macro for tile (function)
macro_rules! Depcrate_imageopstile {
() => {
// Module: crate::imageops
// Provides: {"tile"}
// Dependencies: {}
# [doc = " Tile an image by repeating it multiple times"] # [doc = ""] # [doc = " # Examples"] # [doc = " ```no_run"] # [doc = " use image::RgbaImage;"] # [doc = ""] # [doc = " let mut img = RgbaImage::new(1920, 1080);"] # [doc = " let tile = image::open(\"tile.png\").unwrap();"] # [doc = ""] # [doc = " image::imageops::tile(&mut img, &tile);"] # [doc = " img.save(\"tiled_wallpaper.png\").unwrap();"] # [doc = " ```"] pub fn tile < I , J > (bottom : & mut I , top : & J) where I : GenericImage , J : GenericImageView < Pixel = I :: Pixel > , { for x in (0 .. bottom . width ()) . step_by (top . width () as usize) { for y in (0 .. bottom . height ()) . step_by (top . height () as usize) { overlay (bottom , top , i64 :: from (x) , i64 :: from (y)) ; } } }
};
}
