// Generated macro for impl_42 (impl)
macro_rules! Depcrate_errorimpl_42 {
() => {
// Module: crate::error
// Provides: {"impl_42"}
// Dependencies: {}
impl fmt :: Display for UnsupportedError { fn fmt (& self , fmt : & mut fmt :: Formatter) -> Result < () , fmt :: Error > { match & self . kind { UnsupportedErrorKind :: Format (ImageFormatHint :: Unknown) => { write ! (fmt , "The image format could not be determined" ,) } UnsupportedErrorKind :: Format (format @ ImageFormatHint :: PathExtension (_)) => write ! (fmt , "The file extension {format} was not recognized as an image format" ,) , UnsupportedErrorKind :: Format (format) => { write ! (fmt , "The image format {format} is not supported" ,) } UnsupportedErrorKind :: Color (color) => write ! (fmt , "The encoder or decoder for {} does not support the color type `{:?}`" , self . format , color ,) , UnsupportedErrorKind :: GenericFeature (message) => match & self . format { ImageFormatHint :: Unknown => write ! (fmt , "The decoder does not support the format feature {message}" ,) , other => write ! (fmt , "The decoder for {other} does not support the format features {message}" ,) , } , } } }
};
}
