// Generated macro for uninit_array (function)
macro_rules! Depcrate_de_impl_coreuninit_array {
() => {
// Module: crate::de::impl_core
// Provides: {"uninit_array"}
// Dependencies: {}
# [doc = " Create a new array of `MaybeUninit<T>` items, in an uninitialized state."] # [doc = ""] # [doc = " Note: in a future Rust version this method may become unnecessary"] # [doc = " when Rust allows"] # [doc = " [inline const expressions](https://github.com/rust-lang/rust/issues/76001)."] # [doc = " The example below could then use `let mut buf = [const { MaybeUninit::<u8>::uninit() }; 32];`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```ignore"] # [doc = " #![feature(maybe_uninit_uninit_array, maybe_uninit_extra, maybe_uninit_slice)]"] # [doc = ""] # [doc = " use std::mem::MaybeUninit;"] # [doc = ""] # [doc = " extern \"C\" {"] # [doc = "     fn read_into_buffer(ptr: *mut u8, max_len: usize) -> usize;"] # [doc = " }"] # [doc = ""] # [doc = " /// Returns a (possibly smaller) slice of data that was actually read"] # [doc = " fn read(buf: &mut [MaybeUninit<u8>]) -> &[u8] {"] # [doc = "     unsafe {"] # [doc = "         let len = read_into_buffer(buf.as_mut_ptr() as *mut u8, buf.len());"] # [doc = "         MaybeUninit::slice_assume_init_ref(&buf[..len])"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " let mut buf: [MaybeUninit<u8>; 32] = MaybeUninit::uninit_array();"] # [doc = " let data = read(&mut buf);"] # [doc = " ```"] # [inline (always)] fn uninit_array < T , const LEN : usize > () -> [MaybeUninit < T > ; LEN] { unsafe { MaybeUninit :: < [MaybeUninit < T > ; LEN] > :: uninit () . assume_init () } }
};
}
