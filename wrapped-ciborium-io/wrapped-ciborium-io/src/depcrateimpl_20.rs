// Generated macro for impl_20 (impl)
macro_rules! Depcrateimpl_20 {
() => {
// Module: crate
// Provides: {"impl_20"}
// Dependencies: {}
# [cfg (all (not (feature = "std") , feature = "alloc"))] impl Write for alloc :: vec :: Vec < u8 > { type Error = core :: convert :: Infallible ; # [inline] fn write_all (& mut self , data : & [u8]) -> Result < () , Self :: Error > { self . extend_from_slice (data) ; Ok (()) } # [inline] fn flush (& mut self) -> Result < () , Self :: Error > { Ok (()) } }
};
}
