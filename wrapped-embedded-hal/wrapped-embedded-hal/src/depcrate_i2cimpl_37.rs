// Generated macro for impl_37 (impl)
macro_rules! Depcrate_i2cimpl_37 {
() => {
// Module: crate::i2c
// Provides: {"impl_37"}
// Dependencies: {}
impl core :: fmt :: Display for ErrorKind { # [inline] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self { Self :: Bus => write ! (f , "Bus error occurred") , Self :: ArbitrationLoss => write ! (f , "The arbitration was lost") , Self :: NoAcknowledge (s) => s . fmt (f) , Self :: Overrun => write ! (f , "The peripheral receive buffer was overrun") , Self :: Other => write ! (f , "A different error occurred. The original error may contain more information") , } } }
};
}
