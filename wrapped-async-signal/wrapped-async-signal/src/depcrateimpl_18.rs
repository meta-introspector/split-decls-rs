// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
impl fmt :: Debug for Signals { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { struct RegisteredSignals < 'a > (& 'a HashMap < Signal , SigId >) ; impl fmt :: Debug for RegisteredSignals < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_set () . entries (self . 0 . keys ()) . finish () } } f . debug_struct ("Signals") . field ("notifier" , & self . notifier) . field ("signal_ids" , & RegisteredSignals (& self . signal_ids)) . finish () } }
};
}
