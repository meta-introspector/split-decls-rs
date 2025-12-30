// Generated macro for impl_352 (impl)
macro_rules! Depcrate_util_alphabetimpl_352 {
() => {
// Module: crate::util::alphabet
// Provides: {"impl_352"}
// Dependencies: {}
impl core :: fmt :: Debug for ByteClasses { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { if self . is_singleton () { write ! (f , "ByteClasses(<one-class-per-byte>)") } else { write ! (f , "ByteClasses(") ? ; for (i , class) in self . iter () . enumerate () { if i > 0 { write ! (f , ", ") ? ; } write ! (f , "{:?} => [" , class) ? ; for (start , end) in self . element_ranges (class) { if start == end { write ! (f , "{:?}" , start) ? ; } else { write ! (f , "{:?}-{:?}" , start , end) ? ; } } write ! (f , "]") ? ; } write ! (f , ")") } } }
};
}
