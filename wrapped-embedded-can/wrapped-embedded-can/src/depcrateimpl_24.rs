// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
impl core :: fmt :: Display for ErrorKind { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self { Self :: Overrun => write ! (f , "The peripheral receive buffer was overrun") , Self :: Bit => write ! (f , "Bit value that is monitored differs from the bit value sent") , Self :: Stuff => write ! (f , "Sixth consecutive equal bits detected") , Self :: Crc => write ! (f , "Calculated CRC sequence does not equal the received one") , Self :: Form => write ! (f , "A fixed-form bit field contains one or more illegal bits") , Self :: Acknowledge => write ! (f , "Transmitted frame was not acknowledged") , Self :: Other => write ! (f , "A different error occurred. The original error may contain more information") , } } }
};
}
