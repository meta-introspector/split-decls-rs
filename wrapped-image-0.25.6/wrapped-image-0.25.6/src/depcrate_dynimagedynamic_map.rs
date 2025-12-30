// Generated macro for dynamic_map (macro)
macro_rules! Depcrate_dynimagedynamic_map {
() => {
// Module: crate::dynimage
// Provides: {"dynamic_map"}
// Dependencies: {}
macro_rules ! dynamic_map (($ dynimage : expr , $ image : pat => $ action : expr) => ({ use DynamicImage ::*; match $ dynimage { ImageLuma8 ($ image) => ImageLuma8 ($ action) , ImageLumaA8 ($ image) => ImageLumaA8 ($ action) , ImageRgb8 ($ image) => ImageRgb8 ($ action) , ImageRgba8 ($ image) => ImageRgba8 ($ action) , ImageLuma16 ($ image) => ImageLuma16 ($ action) , ImageLumaA16 ($ image) => ImageLumaA16 ($ action) , ImageRgb16 ($ image) => ImageRgb16 ($ action) , ImageRgba16 ($ image) => ImageRgba16 ($ action) , ImageRgb32F ($ image) => ImageRgb32F ($ action) , ImageRgba32F ($ image) => ImageRgba32F ($ action) , } }) ; ($ dynimage : expr , $ image : pat_param , $ action : expr) => (match $ dynimage { DynamicImage :: ImageLuma8 ($ image) => $ action , DynamicImage :: ImageLumaA8 ($ image) => $ action , DynamicImage :: ImageRgb8 ($ image) => $ action , DynamicImage :: ImageRgba8 ($ image) => $ action , DynamicImage :: ImageLuma16 ($ image) => $ action , DynamicImage :: ImageLumaA16 ($ image) => $ action , DynamicImage :: ImageRgb16 ($ image) => $ action , DynamicImage :: ImageRgba16 ($ image) => $ action , DynamicImage :: ImageRgb32F ($ image) => $ action , DynamicImage :: ImageRgba32F ($ image) => $ action , }) ;) ;
};
}
