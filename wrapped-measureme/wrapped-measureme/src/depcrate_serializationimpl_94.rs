// Generated macro for impl_94 (impl)
macro_rules! Depcrate_serializationimpl_94 {
() => {
// Module: crate::serialization
// Provides: {"impl_94"}
// Dependencies: {}
impl Write for BackingStorage { # [inline] fn write (& mut self , buf : & [u8]) -> std :: io :: Result < usize > { match * self { BackingStorage :: File (ref mut file) => file . write (buf) , BackingStorage :: Memory (ref mut vec) => vec . write (buf) , } } fn flush (& mut self) -> std :: io :: Result < () > { match * self { BackingStorage :: File (ref mut file) => file . flush () , BackingStorage :: Memory (_) => { Ok (()) } } } }
};
}
