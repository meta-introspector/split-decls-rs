// Generated macro for impl_110 (impl)
macro_rules! Depcrate_readimpl_110 {
() => {
// Module: crate::read
// Provides: {"impl_110"}
// Dependencies: {}
impl < 'a > Read < 'a > for MutSliceRead < 'a > { # [inline] fn next (& mut self) -> Result < Option < u8 > > { Ok (if self . index < self . slice . len () { let ch = self . slice [self . index] ; self . index += 1 ; Some (ch) } else { None }) } # [inline] fn peek (& mut self) -> Result < Option < u8 > > { Ok (if self . index < self . slice . len () { Some (self . slice [self . index]) } else { None }) } fn clear_buffer (& mut self) { self . slice = & mut mem :: replace (& mut self . slice , & mut []) [self . index ..] ; self . before += self . index ; self . index = 0 ; self . buffer_end = 0 ; } fn read_to_buffer (& mut self , n : usize) -> Result < () > { let end = self . end (n) ? ; debug_assert ! (self . buffer_end <= self . index , "MutSliceRead invariant violated: scratch buffer exceeds index") ; self . slice [self . buffer_end .. end] . rotate_left (self . index - self . buffer_end) ; self . buffer_end += n ; self . index = end ; Ok (()) } fn take_buffer < 'b > (& 'b mut self) -> EitherLifetime < 'b , 'a > { let (left , right) = mem :: replace (& mut self . slice , & mut []) . split_at_mut (self . index) ; self . slice = right ; self . before += self . index ; self . index = 0 ; let left = & left [.. self . buffer_end] ; self . buffer_end = 0 ; EitherLifetime :: Long (left) } # [inline] fn read_into (& mut self , buf : & mut [u8]) -> Result < () > { let end = self . end (buf . len ()) ? ; buf . copy_from_slice (& self . slice [self . index .. end]) ; self . index = end ; Ok (()) } # [inline] fn discard (& mut self) { self . index += 1 ; } fn offset (& self) -> u64 { (self . before + self . index) as u64 } }
};
}
