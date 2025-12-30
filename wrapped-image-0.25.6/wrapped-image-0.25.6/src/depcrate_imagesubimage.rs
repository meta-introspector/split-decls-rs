// Generated macro for SubImage (struct)
macro_rules! Depcrate_imageSubImage {
() => {
// Module: crate::image
// Provides: {"SubImage"}
// Dependencies: {}
# [doc = " A View into another image"] # [doc = ""] # [doc = " Instances of this struct can be created using:"] # [doc = "   - [`GenericImage::sub_image`] to create a mutable view,"] # [doc = "   - [`GenericImageView::view`] to create an immutable view,"] # [doc = "   - [`SubImage::new`] to instantiate the struct directly."] # [doc = ""] # [doc = " Note that this does _not_ implement `GenericImage`, but it dereferences to one which allows you"] # [doc = " to use it as if it did. See [Design Considerations](#Design-Considerations) below for details."] # [doc = ""] # [doc = " # Design Considerations"] # [doc = ""] # [doc = " For reasons relating to coherence, this is not itself a `GenericImage` or a `GenericImageView`."] # [doc = " In short, we want to reserve the ability of adding traits implemented for _all_ generic images"] # [doc = " but in a different manner for `SubImage`. This may be required to ensure that stacking"] # [doc = " sub-images comes at no double indirect cost."] # [doc = ""] # [doc = " If, ultimately, this is not needed then a directly implementation of `GenericImage` can and"] # [doc = " will get added. This inconvenience may alternatively get resolved if Rust allows some forms of"] # [doc = " specialization, which might make this trick unnecessary and thus also allows for a direct"] # [doc = " implementation."] # [derive (Copy , Clone)] pub struct SubImage < I > { inner : SubImageInner < I > , }
};
}
