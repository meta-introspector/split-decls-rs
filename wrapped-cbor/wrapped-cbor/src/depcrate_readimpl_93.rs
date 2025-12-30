// Generated macro for impl_93 (impl)
macro_rules! Depcrate_readimpl_93 {
() => {
// Module: crate::read
// Provides: {"impl_93"}
// Dependencies: {}
# [cfg (feature = "std")] impl < 'de , R > Read < 'de > for IoRead < R > where R : io :: Read , { # [inline] fn next (& mut self) -> Result < Option < u8 > > { match self . ch . take () { Some (ch) => Ok (Some (ch)) , None => self . next_inner () , } } # [inline] fn peek (& mut self) -> Result < Option < u8 > > { match self . ch { Some (ch) => Ok (Some (ch)) , None => { self . ch = self . next_inner () ? ; Ok (self . ch) } } } fn read_to_buffer (& mut self , mut n : usize) -> Result < () > { self . scratch . reserve (cmp :: min (n , 16 * 1024)) ; if n == 0 { return Ok (()) ; } if let Some (ch) = self . ch . take () { self . scratch . push (ch) ; n -= 1 ; } let transfer_result = { let reference = self . reader . by_ref () ; let mut taken = reference . take (n as u64) ; taken . read_to_end (& mut self . scratch) } ; match transfer_result { Ok (r) if r == n => Ok (()) , Ok (_) => Err (Error :: syntax (ErrorCode :: EofWhileParsingValue , self . offset () ,)) , Err (e) => Err (Error :: io (e)) , } } fn clear_buffer (& mut self) { self . scratch . clear () ; } fn take_buffer < 'a > (& 'a mut self) -> EitherLifetime < 'a , 'de > { EitherLifetime :: Short (& self . scratch) } fn read_into (& mut self , buf : & mut [u8]) -> Result < () > { self . reader . read_exact (buf) . map_err (| e | { if e . kind () == io :: ErrorKind :: UnexpectedEof { Error :: syntax (ErrorCode :: EofWhileParsingValue , self . offset ()) } else { Error :: io (e) } }) } # [inline] fn discard (& mut self) { self . ch = None ; } fn offset (& self) -> u64 { self . reader . offset } }
};
}
