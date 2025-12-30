// Generated macro for impl_8 (impl)
macro_rules! Depcrate_serialimpl_8 {
() => {
// Module: crate::serial
// Provides: {"impl_8"}
// Dependencies: {}
impl core :: fmt :: Display for ErrorKind { # [inline] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self { Self :: Overrun => write ! (f , "The peripheral receive buffer was overrun") , Self :: Parity => write ! (f , "Parity check failed") , Self :: Noise => write ! (f , "Serial line is too noisy to read valid data") , Self :: FrameFormat => write ! (f , "Received data does not conform to the peripheral configuration") , Self :: Other => write ! (f , "A different error occurred. The original error may contain more information") , } } }
};
}
