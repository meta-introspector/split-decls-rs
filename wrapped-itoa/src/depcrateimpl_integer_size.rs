// Generated macro for impl_Integer_size (macro)
macro_rules! Depcrateimpl_Integer_size {
() => {
// Module: crate
// Provides: {"impl_Integer_size"}
// Dependencies: {}
macro_rules ! impl_Integer_size { ($ t : ty as $ primitive : ident # [cfg (target_pointer_width = $ width : literal)]) => { # [cfg (target_pointer_width = $ width)] impl Integer for $ t { const MAX_STR_LEN : usize = <$ primitive as Integer >:: MAX_STR_LEN ; } # [cfg (target_pointer_width = $ width)] impl private :: Sealed for $ t { type Buffer = <$ primitive as private :: Sealed >:: Buffer ; # [inline] # [cfg_attr (feature = "no-panic" , no_panic)] fn write (self , buf : & mut Self :: Buffer) -> & str { (self as $ primitive) . write (buf) } } } ; }
};
}
