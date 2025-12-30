// Generated macro for impl_511 (impl)
macro_rules! Depcrateimpl_511 {
() => {
// Module: crate
// Provides: {"impl_511"}
// Dependencies: {}
impl fmt :: Write for CompactString { fn write_str (& mut self , s : & str) -> fmt :: Result { self . push_str (s) ; Ok (()) } fn write_fmt (mut self : & mut Self , args : fmt :: Arguments < '_ >) -> fmt :: Result { match args . as_str () { Some (s) => { if self . is_empty () && ! self . is_heap_allocated () { * self = Self :: const_new (s) ; } else { self . push_str (s) ; } Ok (()) } None => fmt :: write (& mut self , args) , } } }
};
}
