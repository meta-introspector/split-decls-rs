// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
unsafe impl defmt :: Logger for Logger { fn acquire () { let restore = unsafe { critical_section :: acquire () } ; if TAKEN . load (Ordering :: Relaxed) { panic ! ("defmt logger taken reentrantly") } TAKEN . store (true , Ordering :: Relaxed) ; unsafe { CS_RESTORE = restore } ; unsafe { let encoder : & mut defmt :: Encoder = & mut * core :: ptr :: addr_of_mut ! (ENCODER) ; encoder . start_frame (do_write) } } unsafe fn flush () { while ! stim_0 () . is_fifo_ready () { } asm :: delay (100) ; } unsafe fn release () { unsafe { let encoder : & mut defmt :: Encoder = & mut * core :: ptr :: addr_of_mut ! (ENCODER) ; encoder . end_frame (do_write) ; } TAKEN . store (false , Ordering :: Relaxed) ; let restore = unsafe { CS_RESTORE } ; unsafe { critical_section :: release (restore) ; } } unsafe fn write (bytes : & [u8]) { unsafe { let encoder : & mut defmt :: Encoder = & mut * core :: ptr :: addr_of_mut ! (ENCODER) ; encoder . write (bytes , do_write) ; } } }
};
}
