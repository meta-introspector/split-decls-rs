mkmod!{convert, { 
                getname!(convert);
                getsrc!(convert);
                getpath!(convert);
                get_deps!(convert);
                get_crates!(convert);
                mkinclude!(convert);
                 
            }}
mkmod!{decode, { 
                getname!(decode);
                getsrc!(decode);
                getpath!(decode);
                get_deps!(decode);
                get_crates!(decode);
                mkinclude!(decode);
                 
            }}
mkmod!{methods, { 
                getname!(methods);
                getsrc!(methods);
                getpath!(methods);
                get_deps!(methods);
                get_crates!(methods);
                mkinclude!(methods);
                 
            }}
mkuse!{# [rustfmt :: skip] # [stable (feature = "try_from" , since = "1.34.0")] pub use self :: convert :: CharTryFromError ;}
mkuse!{# [stable (feature = "char_from_str" , since = "1.20.0")] pub use self :: convert :: ParseCharError ;}
mkuse!{# [stable (feature = "decode_utf16" , since = "1.9.0")] pub use self :: decode :: { DecodeUtf16 , DecodeUtf16Error } ;}
mkuse!{# [rustfmt :: skip] # [unstable (feature = "char_internals" , reason = "exposed only for libstd" , issue = "none")] pub use self :: methods :: encode_utf16_raw ;}
mkuse!{# [unstable (feature = "char_internals" , reason = "exposed only for libstd" , issue = "none")] pub use self :: methods :: { encode_utf8_raw , encode_utf8_raw_unchecked } ;}
mkuse!{# [rustfmt :: skip] use crate :: ascii ;}
mkuse!{pub (crate) use self :: methods :: EscapeDebugExtArgs ;}
mkuse!{use crate :: error :: Error ;}
mkuse!{use crate :: escape :: { AlwaysEscaped , EscapeIterInner , MaybeEscaped } ;}
mkuse!{use crate :: fmt :: { self , Write } ;}
mkuse!{use crate :: iter :: { FusedIterator , TrustedLen , TrustedRandomAccess , TrustedRandomAccessNoCoerce } ;}
mkuse!{use crate :: num :: NonZero ;}
mkitem!{const TAG_CONT : u8 = 0b1000_0000 ;}
mkitem!{const TAG_TWO_B : u8 = 0b1100_0000 ;}
mkitem!{const TAG_THREE_B : u8 = 0b1110_0000 ;}
mkitem!{const TAG_FOUR_B : u8 = 0b1111_0000 ;}
mkitem!{const MAX_ONE_B : u32 = 0x80 ;}
mkitem!{const MAX_TWO_B : u32 = 0x800 ;}
mkitem!{const MAX_THREE_B : u32 = 0x10000 ;}
mkitem!{# [doc = " The highest valid code point a `char` can have, `'\\u{10FFFF}'`. Use [`char::MAX`] instead."] # [stable (feature = "rust1" , since = "1.0.0")] pub const MAX : char = char :: MAX ;}
mkitem!{# [doc = " The maximum number of bytes required to [encode](char::encode_utf8) a `char` to"] # [doc = " UTF-8 encoding."] # [unstable (feature = "char_max_len" , issue = "121714")] pub const MAX_LEN_UTF8 : usize = char :: MAX_LEN_UTF8 ;}
mkitem!{# [doc = " The maximum number of two-byte units required to [encode](char::encode_utf16) a `char`"] # [doc = " to UTF-16 encoding."] # [unstable (feature = "char_max_len" , issue = "121714")] pub const MAX_LEN_UTF16 : usize = char :: MAX_LEN_UTF16 ;}
mkitem!{# [doc = " `U+FFFD REPLACEMENT CHARACTER` (�) is used in Unicode to represent a"] # [doc = " decoding error. Use [`char::REPLACEMENT_CHARACTER`] instead."] # [stable (feature = "decode_utf16" , since = "1.9.0")] pub const REPLACEMENT_CHARACTER : char = char :: REPLACEMENT_CHARACTER ;}
mkitem!{# [doc = " The version of [Unicode](https://www.unicode.org/) that the Unicode parts of"] # [doc = " `char` and `str` methods are based on. Use [`char::UNICODE_VERSION`] instead."] # [stable (feature = "unicode_version" , since = "1.45.0")] pub const UNICODE_VERSION : (u8 , u8 , u8) = char :: UNICODE_VERSION ;}

macro_rules! decode_utf16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function decode_utf16 in module {}", module_path!());
    };
}

mkfn!{
    decode_utf16_introspect!();
    # [doc = " Creates an iterator over the UTF-16 encoded code points in `iter`, returning"] # [doc = " unpaired surrogates as `Err`s. Use [`char::decode_utf16`] instead."] # [stable (feature = "decode_utf16" , since = "1.9.0")] # [inline] pub fn decode_utf16 < I : IntoIterator < Item = u16 > > (iter : I) -> DecodeUtf16 < I :: IntoIter > { self :: decode :: decode_utf16 (iter) }
}

macro_rules! from_u32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function from_u32 in module {}", module_path!());
    };
}

mkfn!{
    from_u32_introspect!();
    # [doc = " Converts a `u32` to a `char`. Use [`char::from_u32`] instead."] # [stable (feature = "rust1" , since = "1.0.0")] # [rustc_const_stable (feature = "const_char_convert" , since = "1.67.0")] # [must_use] # [inline] pub const fn from_u32 (i : u32) -> Option < char > { self :: convert :: from_u32 (i) }
}

macro_rules! from_u32_unchecked_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function from_u32_unchecked in module {}", module_path!());
    };
}

mkfn!{
    from_u32_unchecked_introspect!();
    # [doc = " Converts a `u32` to a `char`, ignoring validity. Use [`char::from_u32_unchecked`]"] # [doc = " instead."] # [stable (feature = "char_from_unchecked" , since = "1.5.0")] # [rustc_const_stable (feature = "const_char_from_u32_unchecked" , since = "1.81.0")] # [must_use] # [inline] pub const unsafe fn from_u32_unchecked (i : u32) -> char { unsafe { self :: convert :: from_u32_unchecked (i) } }
}

macro_rules! from_digit_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function from_digit in module {}", module_path!());
    };
}

mkfn!{
    from_digit_introspect!();
    # [doc = " Converts a digit in the given radix to a `char`. Use [`char::from_digit`] instead."] # [stable (feature = "rust1" , since = "1.0.0")] # [rustc_const_stable (feature = "const_char_convert" , since = "1.67.0")] # [must_use] # [inline] pub const fn from_digit (num : u32 , radix : u32) -> Option < char > { self :: convert :: from_digit (num , radix) }
}
mkitem!{mkstruct!{# [doc = " Returns an iterator that yields the hexadecimal Unicode escape of a"] # [doc = " character, as `char`s."] # [doc = ""] # [doc = " This `struct` is created by the [`escape_unicode`] method on [`char`]. See"] # [doc = " its documentation for more."] # [doc = ""] # [doc = " [`escape_unicode`]: char::escape_unicode"] # [derive (Clone , Debug)] # [stable (feature = "rust1" , since = "1.0.0")] pub struct EscapeUnicode (EscapeIterInner < 10 , AlwaysEscaped >) ;}}
mkitem!{mkimpl!{impl EscapeUnicode { # [inline] const fn new (c : char) -> Self { Self (EscapeIterInner :: unicode (c)) } }}}
mkitem!{mkimpl!{# [stable (feature = "rust1" , since = "1.0.0")] impl Iterator for EscapeUnicode { type Item = char ; # [inline] fn next (& mut self) -> Option < char > { self . 0 . next () . map (char :: from) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let n = self . 0 . len () ; (n , Some (n)) } # [inline] fn count (self) -> usize { self . 0 . len () } # [inline] fn last (mut self) -> Option < char > { self . 0 . next_back () . map (char :: from) } # [inline] fn advance_by (& mut self , n : usize) -> Result < () , NonZero < usize > > { self . 0 . advance_by (n) } }}}
mkitem!{mkimpl!{# [stable (feature = "exact_size_escape" , since = "1.11.0")] impl ExactSizeIterator for EscapeUnicode { # [inline] fn len (& self) -> usize { self . 0 . len () } }}}
mkitem!{mkimpl!{# [stable (feature = "fused" , since = "1.26.0")] impl FusedIterator for EscapeUnicode { }}}
mkitem!{mkimpl!{# [stable (feature = "char_struct_display" , since = "1.16.0")] impl fmt :: Display for EscapeUnicode { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . 0 , f) } }}}
mkitem!{mkstruct!{# [doc = " An iterator that yields the literal escape code of a `char`."] # [doc = ""] # [doc = " This `struct` is created by the [`escape_default`] method on [`char`]. See"] # [doc = " its documentation for more."] # [doc = ""] # [doc = " [`escape_default`]: char::escape_default"] # [derive (Clone , Debug)] # [stable (feature = "rust1" , since = "1.0.0")] pub struct EscapeDefault (EscapeIterInner < 10 , AlwaysEscaped >) ;}}
mkitem!{mkimpl!{impl EscapeDefault { # [inline] const fn printable (c : ascii :: Char) -> Self { Self (EscapeIterInner :: ascii (c . to_u8 ())) } # [inline] const fn backslash (c : ascii :: Char) -> Self { Self (EscapeIterInner :: backslash (c)) } # [inline] const fn unicode (c : char) -> Self { Self (EscapeIterInner :: unicode (c)) } }}}
mkitem!{mkimpl!{# [stable (feature = "rust1" , since = "1.0.0")] impl Iterator for EscapeDefault { type Item = char ; # [inline] fn next (& mut self) -> Option < char > { self . 0 . next () . map (char :: from) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let n = self . 0 . len () ; (n , Some (n)) } # [inline] fn count (self) -> usize { self . 0 . len () } # [inline] fn last (mut self) -> Option < char > { self . 0 . next_back () . map (char :: from) } # [inline] fn advance_by (& mut self , n : usize) -> Result < () , NonZero < usize > > { self . 0 . advance_by (n) } }}}
mkitem!{mkimpl!{# [stable (feature = "exact_size_escape" , since = "1.11.0")] impl ExactSizeIterator for EscapeDefault { # [inline] fn len (& self) -> usize { self . 0 . len () } }}}
mkitem!{mkimpl!{# [stable (feature = "fused" , since = "1.26.0")] impl FusedIterator for EscapeDefault { }}}
mkitem!{mkimpl!{# [stable (feature = "char_struct_display" , since = "1.16.0")] impl fmt :: Display for EscapeDefault { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . 0 , f) } }}}
mkitem!{mkstruct!{# [doc = " An iterator that yields the literal escape code of a `char`."] # [doc = ""] # [doc = " This `struct` is created by the [`escape_debug`] method on [`char`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`escape_debug`]: char::escape_debug"] # [stable (feature = "char_escape_debug" , since = "1.20.0")] # [derive (Clone , Debug)] pub struct EscapeDebug (EscapeIterInner < 10 , MaybeEscaped >) ;}}
mkitem!{mkimpl!{impl EscapeDebug { # [inline] const fn printable (chr : char) -> Self { Self (EscapeIterInner :: printable (chr)) } # [inline] const fn backslash (c : ascii :: Char) -> Self { Self (EscapeIterInner :: backslash (c)) } # [inline] const fn unicode (c : char) -> Self { Self (EscapeIterInner :: unicode (c)) } }}}
mkitem!{mkimpl!{# [stable (feature = "char_escape_debug" , since = "1.20.0")] impl Iterator for EscapeDebug { type Item = char ; # [inline] fn next (& mut self) -> Option < char > { self . 0 . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let n = self . len () ; (n , Some (n)) } # [inline] fn count (self) -> usize { self . len () } }}}
mkitem!{mkimpl!{# [stable (feature = "char_escape_debug" , since = "1.20.0")] impl ExactSizeIterator for EscapeDebug { fn len (& self) -> usize { self . 0 . len () } }}}
mkitem!{mkimpl!{# [stable (feature = "fused" , since = "1.26.0")] impl FusedIterator for EscapeDebug { }}}
mkitem!{mkimpl!{# [stable (feature = "char_escape_debug" , since = "1.20.0")] impl fmt :: Display for EscapeDebug { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . 0 , f) } }}}
mkitem!{macro_rules ! casemappingiter_impls { ($ (# [$ attr : meta]) * $ ITER_NAME : ident) => { $ (# [$ attr]) * # [stable (feature = "rust1" , since = "1.0.0")] # [derive (Debug , Clone)] pub struct $ ITER_NAME (CaseMappingIter) ; # [stable (feature = "rust1" , since = "1.0.0")] impl Iterator for $ ITER_NAME { type Item = char ; fn next (& mut self) -> Option < char > { self . 0 . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } fn fold < Acc , Fold > (self , init : Acc , fold : Fold) -> Acc where Fold : FnMut (Acc , Self :: Item) -> Acc , { self . 0 . fold (init , fold) } fn count (self) -> usize { self . 0 . count () } fn last (self) -> Option < Self :: Item > { self . 0 . last () } fn advance_by (& mut self , n : usize) -> Result < () , NonZero < usize >> { self . 0 . advance_by (n) } unsafe fn __iterator_get_unchecked (& mut self , idx : usize) -> Self :: Item { unsafe { self . 0 . __iterator_get_unchecked (idx) } } } # [stable (feature = "case_mapping_double_ended" , since = "1.59.0")] impl DoubleEndedIterator for $ ITER_NAME { fn next_back (& mut self) -> Option < char > { self . 0 . next_back () } fn rfold < Acc , Fold > (self , init : Acc , rfold : Fold) -> Acc where Fold : FnMut (Acc , Self :: Item) -> Acc , { self . 0 . rfold (init , rfold) } fn advance_back_by (& mut self , n : usize) -> Result < () , NonZero < usize >> { self . 0 . advance_back_by (n) } } # [stable (feature = "fused" , since = "1.26.0")] impl FusedIterator for $ ITER_NAME { } # [stable (feature = "exact_size_case_mapping_iter" , since = "1.35.0")] impl ExactSizeIterator for $ ITER_NAME { fn len (& self) -> usize { self . 0 . len () } fn is_empty (& self) -> bool { self . 0 . is_empty () } } # [unstable (feature = "trusted_len" , issue = "37572")] unsafe impl TrustedLen for $ ITER_NAME { } # [doc (hidden)] # [unstable (feature = "std_internals" , issue = "none")] unsafe impl TrustedRandomAccessNoCoerce for $ ITER_NAME { const MAY_HAVE_SIDE_EFFECT : bool = false ; } # [doc (hidden)] # [unstable (feature = "std_internals" , issue = "none")] unsafe impl TrustedRandomAccess for $ ITER_NAME { } # [stable (feature = "char_struct_display" , since = "1.16.0")] impl fmt :: Display for $ ITER_NAME { # [inline] fn fmt (& self , f : & mut fmt :: Formatter <'_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . 0 , f) } } } }}
mkitem!{casemappingiter_impls ! { # [doc = " Returns an iterator that yields the lowercase equivalent of a `char`."] # [doc = ""] # [doc = " This `struct` is created by the [`to_lowercase`] method on [`char`]. See"] # [doc = " its documentation for more."] # [doc = ""] # [doc = " [`to_lowercase`]: char::to_lowercase"] ToLowercase }}
mkitem!{casemappingiter_impls ! { # [doc = " Returns an iterator that yields the uppercase equivalent of a `char`."] # [doc = ""] # [doc = " This `struct` is created by the [`to_uppercase`] method on [`char`]. See"] # [doc = " its documentation for more."] # [doc = ""] # [doc = " [`to_uppercase`]: char::to_uppercase"] ToUppercase }}
mkitem!{mkstruct!{# [derive (Debug , Clone)] struct CaseMappingIter (core :: array :: IntoIter < char , 3 >) ;}}
mkitem!{mkimpl!{impl CaseMappingIter { # [inline] fn new (chars : [char ; 3]) -> CaseMappingIter { let mut iter = chars . into_iter () ; if chars [2] == '\0' { iter . next_back () ; if chars [1] == '\0' { iter . next_back () ; } } CaseMappingIter (iter) } }}}
mkitem!{mkimpl!{impl Iterator for CaseMappingIter { type Item = char ; fn next (& mut self) -> Option < char > { self . 0 . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } fn fold < Acc , Fold > (self , init : Acc , fold : Fold) -> Acc where Fold : FnMut (Acc , Self :: Item) -> Acc , { self . 0 . fold (init , fold) } fn count (self) -> usize { self . 0 . count () } fn last (self) -> Option < Self :: Item > { self . 0 . last () } fn advance_by (& mut self , n : usize) -> Result < () , NonZero < usize > > { self . 0 . advance_by (n) } unsafe fn __iterator_get_unchecked (& mut self , idx : usize) -> Self :: Item { unsafe { self . 0 . __iterator_get_unchecked (idx) } } }}}
mkitem!{mkimpl!{impl DoubleEndedIterator for CaseMappingIter { fn next_back (& mut self) -> Option < char > { self . 0 . next_back () } fn rfold < Acc , Fold > (self , init : Acc , rfold : Fold) -> Acc where Fold : FnMut (Acc , Self :: Item) -> Acc , { self . 0 . rfold (init , rfold) } fn advance_back_by (& mut self , n : usize) -> Result < () , NonZero < usize > > { self . 0 . advance_back_by (n) } }}}
mkitem!{mkimpl!{impl ExactSizeIterator for CaseMappingIter { fn len (& self) -> usize { self . 0 . len () } fn is_empty (& self) -> bool { self . 0 . is_empty () } }}}
mkitem!{mkimpl!{impl FusedIterator for CaseMappingIter { }}}
mkitem!{mkimpl!{unsafe impl TrustedLen for CaseMappingIter { }}}
mkitem!{mkimpl!{unsafe impl TrustedRandomAccessNoCoerce for CaseMappingIter { const MAY_HAVE_SIDE_EFFECT : bool = false ; }}}
mkitem!{mkimpl!{unsafe impl TrustedRandomAccess for CaseMappingIter { }}}
mkitem!{mkimpl!{impl fmt :: Display for CaseMappingIter { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { for c in self . 0 . clone () { f . write_char (c) ? ; } Ok (()) } }}}
mkitem!{mkstruct!{# [doc = " The error type returned when a checked char conversion fails."] # [stable (feature = "u8_from_char" , since = "1.59.0")] # [derive (Debug , Copy , Clone , PartialEq , Eq)] pub struct TryFromCharError (pub (crate) ()) ;}}
mkitem!{mkimpl!{# [stable (feature = "u8_from_char" , since = "1.59.0")] impl fmt :: Display for TryFromCharError { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { "unicode code point out of range" . fmt (fmt) } }}}
mkitem!{mkimpl!{# [stable (feature = "u8_from_char" , since = "1.59.0")] impl Error for TryFromCharError { }}}