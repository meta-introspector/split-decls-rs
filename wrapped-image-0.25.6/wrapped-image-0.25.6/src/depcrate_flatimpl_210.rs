// Generated macro for impl_210 (impl)
macro_rules! Depcrate_flatimpl_210 {
() => {
// Module: crate::flat
// Provides: {"impl_210"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { Error :: TooLarge => write ! (f , "The layout is too large") , Error :: NormalFormRequired (form) => write ! (f , "The layout needs to {}" , match form { NormalForm :: ColumnMajorPacked => "be packed and in column major form" , NormalForm :: ImagePacked => "be fully packed" , NormalForm :: PixelPacked => "have packed pixels" , NormalForm :: RowMajorPacked => "be packed and in row major form" , NormalForm :: Unaliased => "not have any aliasing channels" , }) , Error :: ChannelCountMismatch (layout_channels , pixel_channels) => { write ! (f , "The channel count of the chosen pixel (={pixel_channels}) does agree with the layout (={layout_channels})") } Error :: WrongColor (color) => { write ! (f , "The chosen color type does not match the hint {color:?}") } } } }
};
}
