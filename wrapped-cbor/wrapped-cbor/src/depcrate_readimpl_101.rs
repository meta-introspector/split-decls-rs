// Generated macro for impl_101 (impl)
macro_rules! Depcrate_readimpl_101 {
() => {
// Module: crate::read
// Provides: {"impl_101"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl < 'a > Read < 'a > for SliceRead < 'a > { # [inline] fn next (& mut self) -> Result < Option < u8 > > { Ok (if self . index < self . slice . len () { let ch = self . slice [self . index] ; self . index += 1 ; Some (ch) } else { None }) } # [inline] fn peek (& mut self) -> Result < Option < u8 > > { Ok (if self . index < self . slice . len () { Some (self . slice [self . index]) } else { None }) } fn clear_buffer (& mut self) { self . scratch . clear () ; } fn read_to_buffer (& mut self , n : usize) -> Result < () > { let end = self . end (n) ? ; let slice = & self . slice [self . index .. end] ; self . scratch . extend_from_slice (slice) ; self . index = end ; Ok (()) } # [inline] fn read < 'b > (& 'b mut self , n : usize) -> Result < EitherLifetime < 'b , 'a > > { let end = self . end (n) ? ; let slice = & self . slice [self . index .. end] ; self . index = end ; Ok (EitherLifetime :: Long (slice)) } fn take_buffer < 'b > (& 'b mut self) -> EitherLifetime < 'b , 'a > { EitherLifetime :: Short (& self . scratch) } # [inline] fn read_into (& mut self , buf : & mut [u8]) -> Result < () > { let end = self . end (buf . len ()) ? ; buf . copy_from_slice (& self . slice [self . index .. end]) ; self . index = end ; Ok (()) } # [inline] fn discard (& mut self) { self . index += 1 ; } fn offset (& self) -> u64 { self . index as u64 } }
};
}
