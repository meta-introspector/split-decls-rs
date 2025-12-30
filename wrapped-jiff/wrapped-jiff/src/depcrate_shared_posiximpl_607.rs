// Generated macro for impl_607 (impl)
macro_rules! Depcrate_shared_posiximpl_607 {
() => {
// Module: crate::shared::posix
// Provides: {"impl_607"}
// Dependencies: {}
impl core :: fmt :: Display for PosixDayTime { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "{}" , self . date) ? ; if self . time != PosixTime :: DEFAULT { write ! (f , "/{}" , self . time) ? ; } Ok (()) } }
};
}
