// Generated macro for impl_684 (impl)
macro_rules! Depcrate_util_alphabetimpl_684 {
() => {
// Module: crate::util::alphabet
// Provides: {"impl_684"}
// Dependencies: {}
impl core :: fmt :: Debug for ByteClasses { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { if self . is_singleton () { write ! (f , "ByteClasses({{singletons}})") } else { write ! (f , "ByteClasses(") ? ; for (i , class) in self . iter () . enumerate () { if i > 0 { write ! (f , ", ") ? ; } write ! (f , "{:?} => [" , class . as_usize ()) ? ; for (start , end) in self . element_ranges (class) { if start == end { write ! (f , "{start:?}") ? ; } else { write ! (f , "{start:?}-{end:?}") ? ; } } write ! (f , "]") ? ; } write ! (f , ")") } } }
};
}
