// Generated macro for impl_46 (impl)
macro_rules! Depcrate_errorimpl_46 {
() => {
// Module: crate::error
// Provides: {"impl_46"}
// Dependencies: {}
impl fmt :: Display for EncodingError { fn fmt (& self , fmt : & mut fmt :: Formatter) -> Result < () , fmt :: Error > { match & self . underlying { Some (underlying) => write ! (fmt , "Format error encoding {}:\n{}" , self . format , underlying ,) , None => write ! (fmt , "Format error encoding {}" , self . format ,) , } } }
};
}
