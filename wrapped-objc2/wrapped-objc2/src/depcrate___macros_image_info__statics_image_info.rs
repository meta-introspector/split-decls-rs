// Generated macro for __statics_image_info (macro)
macro_rules! Depcrate___macros_image_info__statics_image_info {
() => {
// Module: crate::__macros::image_info
// Provides: {"__statics_image_info"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] # [cfg (target_vendor = "apple")] macro_rules ! __statics_image_info { ($ hash : expr) => { # [doc = " We always emit the image info tag, since we need it to:"] # [doc = " - End up in the same codegen unit as the other statics below."] # [doc = " - End up in the final binary so it can be read by dyld."] # [doc = ""] # [doc = " If it's not present in the codegen unit, then `ld64` won't set"] # [doc = " `hasObjC` for that specific object file, and in turn it might"] # [doc = " disable processing of the special Objective-C sections (currently"] # [doc = " a category merging pass, in the future who knows what)."] # [doc = ""] # [doc = " Unfortunately however, this leads to duplicated tags - the linker"] # [doc = " reports `__DATA/__objc_imageinfo has unexpectedly large size XXX`,"] # [doc = " but things still seems to work."] # [cfg_attr (not (all (target_os = "macos" , target_arch = "x86")) , link_section = "__DATA,__objc_imageinfo,regular,no_dead_strip")] # [cfg_attr (all (target_os = "macos" , target_arch = "x86") , link_section = "__OBJC,__image_info,regular")] # [export_name = $ crate :: __macros :: concat ! ("\x01L_OBJC_IMAGE_INFO_" , $ hash)] # [used] static _IMAGE_INFO : $ crate :: __macros :: ImageInfo = $ crate :: __macros :: ImageInfo :: system () ; } ; }
};
}
