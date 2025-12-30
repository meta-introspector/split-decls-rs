// Generated macro for impl_209 (impl)
macro_rules! Depcrate_flatimpl_209 {
() => {
// Module: crate::flat
// Provides: {"impl_209"}
// Dependencies: {}
impl From < Error > for ImageError { fn from (error : Error) -> ImageError { # [derive (Debug)] struct NormalFormRequiredError (NormalForm) ; impl fmt :: Display for NormalFormRequiredError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "Required sample buffer in normal form {:?}" , self . 0) } } impl error :: Error for NormalFormRequiredError { } match error { Error :: TooLarge => ImageError :: Parameter (ParameterError :: from_kind (ParameterErrorKind :: DimensionMismatch ,)) , Error :: NormalFormRequired (form) => ImageError :: Decoding (DecodingError :: new (ImageFormatHint :: Unknown , NormalFormRequiredError (form) ,)) , Error :: ChannelCountMismatch (_lc , _pc) => ImageError :: Parameter (ParameterError :: from_kind (ParameterErrorKind :: DimensionMismatch) ,) , Error :: WrongColor (color) => { ImageError :: Unsupported (UnsupportedError :: from_format_and_kind (ImageFormatHint :: Unknown , UnsupportedErrorKind :: Color (color . into ()) ,)) } } } }
};
}
