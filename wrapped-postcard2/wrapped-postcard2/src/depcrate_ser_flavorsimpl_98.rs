// Generated macro for impl_98 (impl)
macro_rules! Depcrate_ser_flavorsimpl_98 {
() => {
// Module: crate::ser::flavors
// Provides: {"impl_98"}
// Dependencies: {}
impl < 'a > Flavor for Slice < 'a > { type Output = & 'a mut [u8] ; # [inline (always)] fn try_push (& mut self , b : u8) -> Result < () > { if self . cursor == self . end { Err (Error :: SerializeBufferFull) } else { unsafe { self . cursor . write (b) ; self . cursor = self . cursor . add (1) ; } Ok (()) } } # [inline (always)] fn try_extend (& mut self , b : & [u8]) -> Result < () > { let remain = (self . end as usize) - (self . cursor as usize) ; let blen = b . len () ; if blen > remain { Err (Error :: SerializeBufferFull) } else { unsafe { core :: ptr :: copy_nonoverlapping (b . as_ptr () , self . cursor , blen) ; self . cursor = self . cursor . add (blen) ; } Ok (()) } } fn finalize (self) -> Result < Self :: Output > { let used = (self . cursor as usize) - (self . start as usize) ; let sli = unsafe { core :: slice :: from_raw_parts_mut (self . start , used) } ; Ok (sli) } }
};
}
