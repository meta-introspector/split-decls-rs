// Generated macro for SubImageInner (struct)
macro_rules! Depcrate_imageSubImageInner {
() => {
// Module: crate::image
// Provides: {"SubImageInner"}
// Dependencies: {}
# [doc = " The inner type of `SubImage` that implements `GenericImage{,View}`."] # [doc = ""] # [doc = " This type is _nominally_ `pub` but it is not exported from the crate. It should be regarded as"] # [doc = " an existential type in any case."] # [derive (Copy , Clone)] pub struct SubImageInner < I > { image : I , xoffset : u32 , yoffset : u32 , xstride : u32 , ystride : u32 , }
};
}
