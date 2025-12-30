// Generated macro for do_dithering (macro)
macro_rules! Depcrate_imageops_coloropsdo_dithering {
() => {
// Module: crate::imageops::colorops
// Provides: {"do_dithering"}
// Dependencies: {}
macro_rules ! do_dithering (($ map : expr , $ image : expr , $ err : expr , $ x : expr , $ y : expr) => ({ let old_pixel = $ image [($ x , $ y)] ; let new_pixel = $ image . get_pixel_mut ($ x , $ y) ; $ map . map_color (new_pixel) ; for ((e , & old) , & new) in $ err . iter_mut () . zip (old_pixel . channels () . iter ()) . zip (new_pixel . channels () . iter ()) { * e = < i16 as From < _ >>:: from (old) - < i16 as From < _ >>:: from (new) } })) ;
};
}
