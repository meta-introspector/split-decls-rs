mkuse!{use crate :: int_overflow :: DebugStrictAdd ;}
mkuse!{use crate :: opaque :: MemDecoder ;}
mkuse!{use crate :: serialize :: Decoder ;}

macro_rules! max_leb128_len_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function max_leb128_len in module {}", module_path!());
    };
}

mkfn!{
    max_leb128_len_introspect!();
    # [doc = " Returns the length of the longest LEB128 encoding for `T`, assuming `T` is an integer type"] pub const fn max_leb128_len < T > () -> usize { (size_of :: < T > () * 8) . div_ceil (7) }
}

macro_rules! largest_max_leb128_len_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function largest_max_leb128_len in module {}", module_path!());
    };
}

mkfn!{
    largest_max_leb128_len_introspect!();
    # [doc = " Returns the length of the longest LEB128 encoding of all supported integer types."] pub const fn largest_max_leb128_len () -> usize { max_leb128_len :: < u128 > () }
}
mkitem!{macro_rules ! impl_write_unsigned_leb128 { ($ fn_name : ident , $ int_ty : ty) => { # [inline] pub fn $ fn_name (out : & mut [u8 ; max_leb128_len ::<$ int_ty > ()] , mut value : $ int_ty) -> usize { let mut i = 0 ; loop { if value < 0x80 { unsafe { * out . get_unchecked_mut (i) = value as u8 ; } i = i . debug_strict_add (1) ; break ; } else { unsafe { * out . get_unchecked_mut (i) = ((value & 0x7f) | 0x80) as u8 ; } value >>= 7 ; i = i . debug_strict_add (1) ; } } i } } ; }}
mkitem!{impl_write_unsigned_leb128 ! (write_u16_leb128 , u16) ;}
mkitem!{impl_write_unsigned_leb128 ! (write_u32_leb128 , u32) ;}
mkitem!{impl_write_unsigned_leb128 ! (write_u64_leb128 , u64) ;}
mkitem!{impl_write_unsigned_leb128 ! (write_u128_leb128 , u128) ;}
mkitem!{impl_write_unsigned_leb128 ! (write_usize_leb128 , usize) ;}
mkitem!{macro_rules ! impl_read_unsigned_leb128 { ($ fn_name : ident , $ int_ty : ty) => { # [inline] pub fn $ fn_name (decoder : & mut MemDecoder <'_ >) -> $ int_ty { let byte = decoder . read_u8 () ; if (byte & 0x80) == 0 { return byte as $ int_ty ; } let mut result = (byte & 0x7F) as $ int_ty ; let mut shift = 7 ; loop { let byte = decoder . read_u8 () ; if (byte & 0x80) == 0 { result |= (byte as $ int_ty) << shift ; return result ; } else { result |= ((byte & 0x7F) as $ int_ty) << shift ; } shift = shift . debug_strict_add (7) ; } } } ; }}
mkitem!{impl_read_unsigned_leb128 ! (read_u16_leb128 , u16) ;}
mkitem!{impl_read_unsigned_leb128 ! (read_u32_leb128 , u32) ;}
mkitem!{impl_read_unsigned_leb128 ! (read_u64_leb128 , u64) ;}
mkitem!{impl_read_unsigned_leb128 ! (read_u128_leb128 , u128) ;}
mkitem!{impl_read_unsigned_leb128 ! (read_usize_leb128 , usize) ;}
mkitem!{macro_rules ! impl_write_signed_leb128 { ($ fn_name : ident , $ int_ty : ty) => { # [inline] pub fn $ fn_name (out : & mut [u8 ; max_leb128_len ::<$ int_ty > ()] , mut value : $ int_ty) -> usize { let mut i = 0 ; loop { let mut byte = (value as u8) & 0x7f ; value >>= 7 ; let more = ! (((value == 0) && ((byte & 0x40) == 0)) || ((value == - 1) && ((byte & 0x40) != 0))) ; if more { byte |= 0x80 ; } unsafe { * out . get_unchecked_mut (i) = byte ; } i = i . debug_strict_add (1) ; if ! more { break ; } } i } } ; }}
mkitem!{impl_write_signed_leb128 ! (write_i16_leb128 , i16) ;}
mkitem!{impl_write_signed_leb128 ! (write_i32_leb128 , i32) ;}
mkitem!{impl_write_signed_leb128 ! (write_i64_leb128 , i64) ;}
mkitem!{impl_write_signed_leb128 ! (write_i128_leb128 , i128) ;}
mkitem!{impl_write_signed_leb128 ! (write_isize_leb128 , isize) ;}
mkitem!{macro_rules ! impl_read_signed_leb128 { ($ fn_name : ident , $ int_ty : ty) => { # [inline] pub fn $ fn_name (decoder : & mut MemDecoder <'_ >) -> $ int_ty { let mut result = 0 ; let mut shift = 0 ; let mut byte ; loop { byte = decoder . read_u8 () ; result |= <$ int_ty >:: from (byte & 0x7F) << shift ; shift = shift . debug_strict_add (7) ; if (byte & 0x80) == 0 { break ; } } if (shift < <$ int_ty >:: BITS) && ((byte & 0x40) != 0) { result |= (! 0 << shift) ; } result } } ; }}
mkitem!{impl_read_signed_leb128 ! (read_i16_leb128 , i16) ;}
mkitem!{impl_read_signed_leb128 ! (read_i32_leb128 , i32) ;}
mkitem!{impl_read_signed_leb128 ! (read_i64_leb128 , i64) ;}
mkitem!{impl_read_signed_leb128 ! (read_i128_leb128 , i128) ;}
mkitem!{impl_read_signed_leb128 ! (read_isize_leb128 , isize) ;}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}