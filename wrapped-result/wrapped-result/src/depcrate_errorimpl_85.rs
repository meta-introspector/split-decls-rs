// Generated macro for impl_85 (impl)
macro_rules! Depcrate_errorimpl_85 {
() => {
// Module: crate::error
// Provides: {"impl_85"}
// Dependencies: {}
impl core :: fmt :: Display for Error { fn fmt (& self , fmt : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { let message = self . message () ; if message . is_empty () { core :: write ! (fmt , "{}" , self . code ()) } else { core :: write ! (fmt , "{} ({})" , message , self . code ()) } } }
};
}
