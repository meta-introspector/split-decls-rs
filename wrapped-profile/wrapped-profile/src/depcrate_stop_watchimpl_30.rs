// Generated macro for impl_30 (impl)
macro_rules! Depcrate_stop_watchimpl_30 {
() => {
// Module: crate::stop_watch
// Provides: {"impl_30"}
// Dependencies: {}
impl fmt :: Display for StopWatchSpan { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{:.2?}" , self . time) ? ; if let Some (mut instructions) = self . instructions { let mut prefix = "" ; if instructions > 10000 { instructions /= 1000 ; prefix = "k" ; } if instructions > 10000 { instructions /= 1000 ; prefix = "m" ; } if instructions > 10000 { instructions /= 1000 ; prefix = "g" ; } write ! (f , ", {instructions}{prefix}instr") ? ; } write ! (f , ", {}" , self . memory) ? ; Ok (()) } }
};
}
