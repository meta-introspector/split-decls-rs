// Generated macro for impl_104 (impl)
macro_rules! Depcrate_errorimpl_104 {
() => {
// Module: crate::error
// Provides: {"impl_104"}
// Dependencies: {}
impl < 'a > Display for JsonUnexpected < 'a > { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { match self . 0 { de :: Unexpected :: Unit => formatter . write_str ("null") , de :: Unexpected :: Float (value) => write ! (formatter , "floating point `{}`" , ryu :: Buffer :: new () . format (value) ,) , unexp => Display :: fmt (& unexp , formatter) , } } }
};
}
