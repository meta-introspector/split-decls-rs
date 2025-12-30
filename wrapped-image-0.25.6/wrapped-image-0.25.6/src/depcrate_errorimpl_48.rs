// Generated macro for impl_48 (impl)
macro_rules! Depcrate_errorimpl_48 {
() => {
// Module: crate::error
// Provides: {"impl_48"}
// Dependencies: {}
impl fmt :: Display for DecodingError { fn fmt (& self , fmt : & mut fmt :: Formatter) -> Result < () , fmt :: Error > { match & self . underlying { None => match self . format { ImageFormatHint :: Unknown => write ! (fmt , "Format error") , _ => write ! (fmt , "Format error decoding {}" , self . format) , } , Some (underlying) => { write ! (fmt , "Format error decoding {}: {}" , self . format , underlying) } } } }
};
}
