// Generated macro for impl_41 (impl)
macro_rules! Depcrate_shared_posiximpl_41 {
() => {
// Module: crate::shared::posix
// Provides: {"impl_41"}
// Dependencies: {}
impl core :: fmt :: Display for PosixDayTime { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "{}" , self . date) ? ; if self . time != PosixTime :: DEFAULT { write ! (f , "/{}" , self . time) ? ; } Ok (()) } }
};
}
