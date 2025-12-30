// Generated macro for impl_553 (impl)
macro_rules! Depcrate_fmtimpl_553 {
() => {
// Module: crate::fmt
// Provides: {"impl_553"}
// Dependencies: {}
impl < 'i , V : core :: fmt :: Debug > core :: fmt :: Debug for Parsed < 'i , V > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { f . debug_struct ("Parsed") . field ("value" , & self . value) . field ("input" , & escape :: Bytes (self . input)) . finish () } }
};
}
