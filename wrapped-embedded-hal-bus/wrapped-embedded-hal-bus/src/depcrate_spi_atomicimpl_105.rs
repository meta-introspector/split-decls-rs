// Generated macro for impl_105 (impl)
macro_rules! Depcrate_spi_atomicimpl_105 {
() => {
// Module: crate::spi::atomic
// Provides: {"impl_105"}
// Dependencies: {}
impl < Word : Copy + 'static , BUS , CS , D > SpiDevice < Word > for AtomicDevice < '_ , BUS , CS , D > where BUS : SpiBus < Word > , CS : OutputPin , D : DelayNs , { # [inline] fn transaction (& mut self , operations : & mut [Operation < '_ , Word >]) -> Result < () , Self :: Error > { self . bus . busy . compare_exchange (false , true , core :: sync :: atomic :: Ordering :: SeqCst , core :: sync :: atomic :: Ordering :: SeqCst ,) . map_err (| _ | AtomicError :: Busy) ? ; let bus = unsafe { & mut * self . bus . bus . get () } ; let result = transaction (operations , bus , & mut self . delay , & mut self . cs) ; self . bus . busy . store (false , core :: sync :: atomic :: Ordering :: SeqCst) ; result . map_err (AtomicError :: Other) } }
};
}
