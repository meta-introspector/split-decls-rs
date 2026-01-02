mkuse!{use std :: { ascii , fmt } ;}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkitem!{pub const MAX_BASE : usize = 64 ;}
mkitem!{pub const ALPHANUMERIC_ONLY : usize = 62 ;}
mkitem!{pub const CASE_INSENSITIVE : usize = 36 ;}
mkitem!{const BASE_64 : [ascii :: Char ; MAX_BASE] = { let bytes = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ@$" ; let Some (ascii) = bytes . as_ascii () else { panic ! () } ; * ascii } ;}
mkitem!{mkstruct!{pub struct BaseNString { start : usize , buf : [ascii :: Char ; 128] , }}}
mkitem!{mkimpl!{impl std :: ops :: Deref for BaseNString { type Target = str ; fn deref (& self) -> & str { self . buf [self . start ..] . as_str () } }}}
mkitem!{mkimpl!{impl AsRef < str > for BaseNString { fn as_ref (& self) -> & str { self . buf [self . start ..] . as_str () } }}}
mkitem!{mkimpl!{impl fmt :: Display for BaseNString { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (self) } }}}
mkitem!{mktrait!{pub trait ToBaseN : Into < u128 > { fn encoded_len (base : usize) -> usize ; fn to_base_fixed_len (self , base : usize) -> BaseNString { let mut encoded = self . to_base (base) ; encoded . start = encoded . buf . len () - Self :: encoded_len (base) ; encoded } fn to_base (self , base : usize) -> BaseNString { let mut output = [ascii :: Char :: Digit0 ; 128] ; let mut n : u128 = self . into () ; let mut index = output . len () ; loop { index -= 1 ; output [index] = BASE_64 [(n % base as u128) as usize] ; n /= base as u128 ; if n == 0 { break ; } } assert_eq ! (n , 0) ; BaseNString { start : index , buf : output } } }}}
mkitem!{mkimpl!{impl ToBaseN for u128 { fn encoded_len (base : usize) -> usize { let mut max = u128 :: MAX ; let mut len = 0 ; while max > 0 { len += 1 ; max /= base as u128 ; } len } }}}
mkitem!{mkimpl!{impl ToBaseN for u64 { fn encoded_len (base : usize) -> usize { let mut max = u64 :: MAX ; let mut len = 0 ; while max > 0 { len += 1 ; max /= base as u64 ; } len } }}}
mkitem!{mkimpl!{impl ToBaseN for u32 { fn encoded_len (base : usize) -> usize { let mut max = u32 :: MAX ; let mut len = 0 ; while max > 0 { len += 1 ; max /= base as u32 ; } len } }}}