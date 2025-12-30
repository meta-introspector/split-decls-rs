// Generated macro for impl_44 (impl)
macro_rules! Depcrate_rustcrypto_implimpl_44 {
() => {
// Module: crate::rustcrypto_impl
// Provides: {"impl_44"}
// Dependencies: {}
impl Buffer { fn try_apply_keystream < EnableWide : AsBool > (& mut self , mut data : & mut [u8] , drounds : u32 ,) -> Result < () , () > { if self . have < 0 { self . state . refill (drounds , & mut self . out) ; self . have += BLOCK as i8 ; self . len -= 1 ; } let mut have = self . have as usize ; let have_ready = cmp :: min (have , data . len ()) ; let datalen = (data . len () - have_ready) as u64 ; let blocks_needed = datalen / BLOCK64 + u64 :: from (datalen % BLOCK64 != 0) ; let (l , o) = self . len . overflowing_sub (blocks_needed) ; if o && ! self . fresh { return Err (()) ; } self . len = l ; self . fresh &= blocks_needed == 0 ; let (d0 , d1) = data . split_at_mut (have_ready) ; for (data_b , key_b) in d0 . iter_mut () . zip (& self . out [(BLOCK - have) ..]) { * data_b ^= * key_b ; } data = d1 ; have -= have_ready ; if EnableWide :: BOOL { let (d0 , d1) = data . split_at_mut (data . len () & ! (BUFSZ - 1)) ; for dd in d0 . chunks_exact_mut (BUFSZ) { let mut buf = [0 ; BUFSZ] ; self . state . refill4 (drounds , & mut buf) ; for (data_b , key_b) in dd . iter_mut () . zip (buf . iter ()) { * data_b ^= * key_b ; } } data = d1 ; } for dd in data . chunks_mut (BLOCK) { self . state . refill (drounds , & mut self . out) ; for (data_b , key_b) in dd . iter_mut () . zip (self . out . iter ()) { * data_b ^= * key_b ; } have = BLOCK - dd . len () ; } self . have = have as i8 ; Ok (()) } }
};
}
