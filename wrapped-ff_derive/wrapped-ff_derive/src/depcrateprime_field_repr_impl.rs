// Generated macro for prime_field_repr_impl (function)
macro_rules! Depcrateprime_field_repr_impl {
() => {
// Module: crate
// Provides: {"prime_field_repr_impl"}
// Dependencies: {}
fn prime_field_repr_impl (repr : & syn :: Ident , endianness : & ReprEndianness , bytes : usize ,) -> proc_macro2 :: TokenStream { let repr_iter_be = endianness . iter_be () ; quote ! { # [derive (Copy , Clone)] pub struct # repr (pub [u8 ; # bytes]) ; impl :: ff :: derive :: subtle :: ConstantTimeEq for # repr { fn ct_eq (& self , other : &# repr) -> :: ff :: derive :: subtle :: Choice { self . 0 . iter () . zip (other . 0 . iter ()) . map (| (a , b) | a . ct_eq (b)) . fold (1 . into () , | acc , x | acc & x) } } impl :: core :: cmp :: PartialEq for # repr { fn eq (& self , other : &# repr) -> bool { use :: ff :: derive :: subtle :: ConstantTimeEq ; self . ct_eq (other) . into () } } impl :: core :: cmp :: Eq for # repr { } impl :: core :: default :: Default for # repr { fn default () -> # repr { # repr ([0u8 ; # bytes]) } } impl :: core :: fmt :: Debug for # repr { fn fmt (& self , f : & mut :: core :: fmt :: Formatter) -> :: core :: fmt :: Result { write ! (f , "0x") ?; for i in # repr_iter_be { write ! (f , "{:02x}" , * i) ?; } Ok (()) } } impl AsRef < [u8] > for # repr { # [inline (always)] fn as_ref (& self) -> & [u8] { & self . 0 } } impl AsMut < [u8] > for # repr { # [inline (always)] fn as_mut (& mut self) -> & mut [u8] { & mut self . 0 } } } }
};
}
