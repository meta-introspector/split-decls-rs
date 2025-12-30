// Generated macro for impl_45 (impl)
macro_rules! Depcrateimpl_45 {
() => {
// Module: crate
// Provides: {"impl_45"}
// Dependencies: {}
impl Arena < u8 > { # [doc = " Allocates a string slice and returns a mutable reference to it."] # [doc = ""] # [doc = " This is on `Arena<u8>`, because string slices use byte slices (`[u8]`) as their backing"] # [doc = " storage."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use typed_arena::Arena;"] # [doc = ""] # [doc = " let arena: Arena<u8> = Arena::new();"] # [doc = " let hello = arena.alloc_str(\"Hello world\");"] # [doc = " assert_eq!(\"Hello world\", hello);"] # [doc = " ```"] # [inline] pub fn alloc_str (& self , s : & str) -> & mut str { let buffer = self . alloc_extend (s . bytes ()) ; unsafe { str :: from_utf8_unchecked_mut (buffer) } } }
};
}
