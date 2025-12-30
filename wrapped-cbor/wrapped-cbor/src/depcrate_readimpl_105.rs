// Generated macro for impl_105 (impl)
macro_rules! Depcrate_readimpl_105 {
() => {
// Module: crate::read
// Provides: {"impl_105"}
// Dependencies: {}
impl < 'a , 'b > Read < 'a > for SliceReadFixed < 'a , 'b > { # [inline] fn next (& mut self) -> Result < Option < u8 > > { Ok (if self . index < self . slice . len () { let ch = self . slice [self . index] ; self . index += 1 ; Some (ch) } else { None }) } # [inline] fn peek (& mut self) -> Result < Option < u8 > > { Ok (if self . index < self . slice . len () { Some (self . slice [self . index]) } else { None }) } fn clear_buffer (& mut self) { self . scratch_index = 0 ; } fn read_to_buffer (& mut self , n : usize) -> Result < () > { let end = self . end (n) ? ; let scratch_end = self . scratch_end (n) ? ; let slice = & self . slice [self . index .. end] ; self . scratch [self . scratch_index .. scratch_end] . copy_from_slice (& slice) ; self . index = end ; self . scratch_index = scratch_end ; Ok (()) } fn read < 'c > (& 'c mut self , n : usize) -> Result < EitherLifetime < 'c , 'a > > { let end = self . end (n) ? ; let slice = & self . slice [self . index .. end] ; self . index = end ; Ok (EitherLifetime :: Long (slice)) } fn take_buffer < 'c > (& 'c mut self) -> EitherLifetime < 'c , 'a > { EitherLifetime :: Short (& self . scratch [0 .. self . scratch_index]) } # [inline] fn read_into (& mut self , buf : & mut [u8]) -> Result < () > { let end = self . end (buf . len ()) ? ; buf . copy_from_slice (& self . slice [self . index .. end]) ; self . index = end ; Ok (()) } # [inline] fn discard (& mut self) { self . index += 1 ; } fn offset (& self) -> u64 { self . index as u64 } }
};
}
