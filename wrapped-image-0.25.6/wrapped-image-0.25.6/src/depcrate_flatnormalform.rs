// Generated macro for NormalForm (enum)
macro_rules! Depcrate_flatNormalForm {
() => {
// Module: crate::flat
// Provides: {"NormalForm"}
// Dependencies: {}
# [doc = " Different normal forms of buffers."] # [doc = ""] # [doc = " A normal form is an unaliased buffer with some additional constraints.  The `ÌmageBuffer` uses"] # [doc = " row major form with packed samples."] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub enum NormalForm { # [doc = " No pixel aliases another."] # [doc = ""] # [doc = " Unaliased also guarantees that all index calculations in the image bounds using"] # [doc = " `dim_index*dim_stride` (such as `x*width_stride + y*height_stride`) do not overflow."] Unaliased , # [doc = " At least pixels are packed."] # [doc = ""] # [doc = " Images of these types can wrap `[T]`-slices into the standard color types. This is a"] # [doc = " precondition for `GenericImage` which requires by-reference access to pixels."] PixelPacked , # [doc = " All samples are packed."] # [doc = ""] # [doc = " This is orthogonal to `PixelPacked`. It requires that there are no holes in the image but"] # [doc = " it is not necessary that the pixel samples themselves are adjacent. An example of this"] # [doc = " behaviour is a planar image layout."] ImagePacked , # [doc = " The samples are in row-major form and all samples are packed."] # [doc = ""] # [doc = " In addition to `PixelPacked` and `ImagePacked` this also asserts that the pixel matrix is"] # [doc = " in row-major form."] RowMajorPacked , # [doc = " The samples are in column-major form and all samples are packed."] # [doc = ""] # [doc = " In addition to `PixelPacked` and `ImagePacked` this also asserts that the pixel matrix is"] # [doc = " in column-major form."] ColumnMajorPacked , }
};
}
