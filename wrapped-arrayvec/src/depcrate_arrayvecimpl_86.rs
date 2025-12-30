// Generated macro for impl_86 (impl)
macro_rules! Depcrate_arrayvecimpl_86 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_86"}
// Dependencies: {}
# [cfg (feature = "zeroize")] # [doc = " \"Best efforts\" zeroing of the `ArrayVec`'s buffer when the `zeroize` feature is enabled."] # [doc = ""] # [doc = " The length is set to 0, and the buffer is dropped and zeroized."] # [doc = " Cannot ensure that previous moves of the `ArrayVec` did not leave values on the stack."] # [doc = ""] # [doc = " ```"] # [doc = " use arrayvec::ArrayVec;"] # [doc = " use zeroize::Zeroize;"] # [doc = " let mut array = ArrayVec::from([1, 2, 3]);"] # [doc = " array.zeroize();"] # [doc = " assert_eq!(array.len(), 0);"] # [doc = " let data = unsafe { core::slice::from_raw_parts(array.as_ptr(), array.capacity()) };"] # [doc = " assert_eq!(data, [0, 0, 0]);"] # [doc = " ```"] impl < Z : zeroize :: Zeroize , const CAP : usize > zeroize :: Zeroize for ArrayVec < Z , CAP > { fn zeroize (& mut self) { self . iter_mut () . zeroize () ; self . clear () ; self . xs . zeroize () ; } }
};
}
