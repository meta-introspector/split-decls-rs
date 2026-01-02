mkuse!{use crate :: range :: RangeFull ;}
mkitem!{mktrait!{# [doc = " A source of randomness."] # [unstable (feature = "random" , issue = "130703")] pub trait RandomSource { # [doc = " Fills `bytes` with random bytes."] # [doc = ""] # [doc = " Note that calling `fill_bytes` multiple times is not equivalent to calling `fill_bytes` once"] # [doc = " with a larger buffer. A `RandomSource` is allowed to return different bytes for those two"] # [doc = " cases. For instance, this allows a `RandomSource` to generate a word at a time and throw"] # [doc = " part of it away if not needed."] fn fill_bytes (& mut self , bytes : & mut [u8]) ; }}}
mkitem!{mktrait!{# [doc = " A trait representing a distribution of random values for a type."] # [unstable (feature = "random" , issue = "130703")] pub trait Distribution < T > { # [doc = " Samples a random value from the distribution, using the specified random source."] fn sample (& self , source : & mut (impl RandomSource + ? Sized)) -> T ; }}}
mkitem!{mkimpl!{impl < T , DT : Distribution < T > > Distribution < T > for & DT { fn sample (& self , source : & mut (impl RandomSource + ? Sized)) -> T { (* self) . sample (source) } }}}
mkitem!{mkimpl!{impl Distribution < bool > for RangeFull { fn sample (& self , source : & mut (impl RandomSource + ? Sized)) -> bool { let byte : u8 = RangeFull . sample (source) ; byte & 1 == 1 } }}}
mkitem!{macro_rules ! impl_primitive { ($ t : ty) => { impl Distribution <$ t > for RangeFull { fn sample (& self , source : & mut (impl RandomSource + ? Sized)) -> $ t { let mut bytes = (0 as $ t) . to_ne_bytes () ; source . fill_bytes (& mut bytes) ; <$ t >:: from_ne_bytes (bytes) } } } ; }}
mkitem!{impl_primitive ! (u8) ;}
mkitem!{impl_primitive ! (i8) ;}
mkitem!{impl_primitive ! (u16) ;}
mkitem!{impl_primitive ! (i16) ;}
mkitem!{impl_primitive ! (u32) ;}
mkitem!{impl_primitive ! (i32) ;}
mkitem!{impl_primitive ! (u64) ;}
mkitem!{impl_primitive ! (i64) ;}
mkitem!{impl_primitive ! (u128) ;}
mkitem!{impl_primitive ! (i128) ;}
mkitem!{impl_primitive ! (usize) ;}
mkitem!{impl_primitive ! (isize) ;}