// Generated macro for impl_55 (impl)
macro_rules! Depcrate_array_stringimpl_55 {
() => {
// Module: crate::array_string
// Provides: {"impl_55"}
// Dependencies: {}
# [cfg (feature = "zeroize")] # [doc = " \"Best efforts\" zeroing of the `ArrayString`'s buffer when the `zeroize` feature is enabled."] # [doc = ""] # [doc = " The length is set to 0, and the buffer is dropped and zeroized."] # [doc = " Cannot ensure that previous moves of the `ArrayString` did not leave values on the stack."] # [doc = ""] # [doc = " ```"] # [doc = " use arrayvec::ArrayString;"] # [doc = " use zeroize::Zeroize;"] # [doc = " let mut string = ArrayString::<6>::from(\"foobar\").unwrap();"] # [doc = " string.zeroize();"] # [doc = " assert_eq!(string.len(), 0);"] # [doc = " unsafe { string.set_len(string.capacity()) };"] # [doc = " assert_eq!(&*string, \"\\0\\0\\0\\0\\0\\0\");"] # [doc = " ```"] impl < const CAP : usize > zeroize :: Zeroize for ArrayString < CAP > { fn zeroize (& mut self) { self . clear () ; self . xs . zeroize () ; } }
};
}
