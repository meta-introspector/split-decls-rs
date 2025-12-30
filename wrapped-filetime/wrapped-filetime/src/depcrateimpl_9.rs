// Generated macro for impl_9 (impl)
macro_rules! Depcrateimpl_9 {
() => {
// Module: crate
// Provides: {"impl_9"}
// Dependencies: {}
impl fmt :: Display for FileTime { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "{}.{:09}s" , self . seconds , self . nanos) } }
};
}
