// Generated macro for impl_77 (impl)
macro_rules! Depcrate_deimpl_77 {
() => {
// Module: crate::de
// Provides: {"impl_77"}
// Dependencies: {}
impl < T , const N : usize > BorshDeserialize for [T ; N] where T : BorshDeserialize , { # [inline] fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { struct ArrayDropGuard < T , const N : usize > { buffer : [MaybeUninit < T > ; N] , init_count : usize , } impl < T , const N : usize > Drop for ArrayDropGuard < T , N > { fn drop (& mut self) { let init_range = & mut self . buffer [.. self . init_count] ; unsafe { core :: ptr :: drop_in_place (init_range as * mut _ as * mut [T]) ; } ; } } impl < T , const N : usize > ArrayDropGuard < T , N > { unsafe fn transmute_to_array (mut self) -> [T ; N] { debug_assert_eq ! (self . init_count , N) ; self . init_count = 0 ; core :: ptr :: read (& self . buffer as * const _ as * const [T ; N]) } fn fill_buffer (& mut self , mut f : impl FnMut () -> Result < T >) -> Result < () > { for elem in self . buffer . iter_mut () { elem . write (f () ?) ; self . init_count += 1 ; } Ok (()) } } if let Some (arr) = T :: array_from_reader (reader) ? { Ok (arr) } else { let mut result = ArrayDropGuard { buffer : unsafe { MaybeUninit :: uninit () . assume_init () } , init_count : 0 , } ; result . fill_buffer (| | T :: deserialize_reader (reader)) ? ; Ok (unsafe { result . transmute_to_array () }) } } }
};
}
