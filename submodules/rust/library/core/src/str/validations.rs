mkuse!{use super :: Utf8Error ;}
mkuse!{use crate :: intrinsics :: const_eval_select ;}

macro_rules! utf8_first_byte_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function utf8_first_byte in module {}", module_path!());
    };
}

mkfn!{
    utf8_first_byte_introspect!();
    # [doc = " Returns the initial codepoint accumulator for the first byte."] # [doc = " The first byte is special, only want bottom 5 bits for width 2, 4 bits"] # [doc = " for width 3, and 3 bits for width 4."] # [inline] const fn utf8_first_byte (byte : u8 , width : u32) -> u32 { (byte & (0x7F >> width)) as u32 }
}

macro_rules! utf8_acc_cont_byte_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function utf8_acc_cont_byte in module {}", module_path!());
    };
}

mkfn!{
    utf8_acc_cont_byte_introspect!();
    # [doc = " Returns the value of `ch` updated with continuation byte `byte`."] # [inline] const fn utf8_acc_cont_byte (ch : u32 , byte : u8) -> u32 { (ch << 6) | (byte & CONT_MASK) as u32 }
}

macro_rules! utf8_is_cont_byte_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function utf8_is_cont_byte in module {}", module_path!());
    };
}

mkfn!{
    utf8_is_cont_byte_introspect!();
    # [doc = " Checks whether the byte is a UTF-8 continuation byte (i.e., starts with the"] # [doc = " bits `10`)."] # [inline] pub (super) const fn utf8_is_cont_byte (byte : u8) -> bool { (byte as i8) < - 64 }
}

macro_rules! next_code_point_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function next_code_point in module {}", module_path!());
    };
}

mkfn!{
    next_code_point_introspect!();
    # [doc = " Reads the next code point out of a byte iterator (assuming a"] # [doc = " UTF-8-like encoding)."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `bytes` must produce a valid UTF-8-like (UTF-8 or WTF-8) string"] # [unstable (feature = "str_internals" , issue = "none")] # [inline] pub unsafe fn next_code_point < 'a , I : Iterator < Item = & 'a u8 > > (bytes : & mut I) -> Option < u32 > { let x = * bytes . next () ? ; if x < 128 { return Some (x as u32) ; } let init = utf8_first_byte (x , 2) ; let y = unsafe { * bytes . next () . unwrap_unchecked () } ; let mut ch = utf8_acc_cont_byte (init , y) ; if x >= 0xE0 { let z = unsafe { * bytes . next () . unwrap_unchecked () } ; let y_z = utf8_acc_cont_byte ((y & CONT_MASK) as u32 , z) ; ch = init << 12 | y_z ; if x >= 0xF0 { let w = unsafe { * bytes . next () . unwrap_unchecked () } ; ch = (init & 7) << 18 | utf8_acc_cont_byte (y_z , w) ; } } Some (ch) }
}

macro_rules! next_code_point_reverse_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function next_code_point_reverse in module {}", module_path!());
    };
}

mkfn!{
    next_code_point_reverse_introspect!();
    # [doc = " Reads the last code point out of a byte iterator (assuming a"] # [doc = " UTF-8-like encoding)."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `bytes` must produce a valid UTF-8-like (UTF-8 or WTF-8) string"] # [inline] pub (super) unsafe fn next_code_point_reverse < 'a , I > (bytes : & mut I) -> Option < u32 > where I : DoubleEndedIterator < Item = & 'a u8 > , { let w = match * bytes . next_back () ? { next_byte if next_byte < 128 => return Some (next_byte as u32) , back_byte => back_byte , } ; let mut ch ; let z = unsafe { * bytes . next_back () . unwrap_unchecked () } ; ch = utf8_first_byte (z , 2) ; if utf8_is_cont_byte (z) { let y = unsafe { * bytes . next_back () . unwrap_unchecked () } ; ch = utf8_first_byte (y , 3) ; if utf8_is_cont_byte (y) { let x = unsafe { * bytes . next_back () . unwrap_unchecked () } ; ch = utf8_first_byte (x , 4) ; ch = utf8_acc_cont_byte (ch , y) ; } ch = utf8_acc_cont_byte (ch , z) ; } ch = utf8_acc_cont_byte (ch , w) ; Some (ch) }
}
mkitem!{const NONASCII_MASK : usize = usize :: repeat_u8 (0x80) ;}

macro_rules! contains_nonascii_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function contains_nonascii in module {}", module_path!());
    };
}

mkfn!{
    contains_nonascii_introspect!();
    # [doc = " Returns `true` if any byte in the word `x` is nonascii (>= 128)."] # [inline] const fn contains_nonascii (x : usize) -> bool { (x & NONASCII_MASK) != 0 }
}

macro_rules! run_utf8_validation_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function run_utf8_validation in module {}", module_path!());
    };
}

mkfn!{
    run_utf8_validation_introspect!();
    # [doc = " Walks through `v` checking that it's a valid UTF-8 sequence,"] # [doc = " returning `Ok(())` in that case, or, if it is invalid, `Err(err)`."] # [inline (always)] # [rustc_allow_const_fn_unstable (const_eval_select)] pub (super) const fn run_utf8_validation (v : & [u8]) -> Result < () , Utf8Error > { let mut index = 0 ; let len = v . len () ; const USIZE_BYTES : usize = size_of :: < usize > () ; let ascii_block_size = 2 * USIZE_BYTES ; let blocks_end = if len >= ascii_block_size { len - ascii_block_size + 1 } else { 0 } ; let align = const_eval_select ! (@ capture { v : & [u8] } -> usize : if const { usize :: MAX } else { v . as_ptr () . align_offset (USIZE_BYTES) }) ; while index < len { let old_offset = index ; macro_rules ! err { ($ error_len : expr) => { return Err (Utf8Error { valid_up_to : old_offset , error_len : $ error_len }) } ; } macro_rules ! next { () => { { index += 1 ; if index >= len { err ! (None) } v [index] } } ; } let first = v [index] ; if first >= 128 { let w = utf8_char_width (first) ; match w { 2 => { if next ! () as i8 >= - 64 { err ! (Some (1)) } } 3 => { match (first , next ! ()) { (0xE0 , 0xA0 ..= 0xBF) | (0xE1 ..= 0xEC , 0x80 ..= 0xBF) | (0xED , 0x80 ..= 0x9F) | (0xEE ..= 0xEF , 0x80 ..= 0xBF) => { } _ => err ! (Some (1)) , } if next ! () as i8 >= - 64 { err ! (Some (2)) } } 4 => { match (first , next ! ()) { (0xF0 , 0x90 ..= 0xBF) | (0xF1 ..= 0xF3 , 0x80 ..= 0xBF) | (0xF4 , 0x80 ..= 0x8F) => { } _ => err ! (Some (1)) , } if next ! () as i8 >= - 64 { err ! (Some (2)) } if next ! () as i8 >= - 64 { err ! (Some (3)) } } _ => err ! (Some (1)) , } index += 1 ; } else { if align != usize :: MAX && align . wrapping_sub (index) . is_multiple_of (USIZE_BYTES) { let ptr = v . as_ptr () ; while index < blocks_end { unsafe { let block = ptr . add (index) as * const usize ; let zu = contains_nonascii (* block) ; let zv = contains_nonascii (* block . add (1)) ; if zu || zv { break ; } } index += ascii_block_size ; } while index < len && v [index] < 128 { index += 1 ; } } else { index += 1 ; } } } Ok (()) }
}
mkitem!{const UTF8_CHAR_WIDTH : & [u8 ; 256] = & [1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 2 , 3 , 3 , 3 , 3 , 3 , 3 , 3 , 3 , 3 , 3 , 3 , 3 , 3 , 3 , 3 , 3 , 4 , 4 , 4 , 4 , 4 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ,] ;}

macro_rules! utf8_char_width_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function utf8_char_width in module {}", module_path!());
    };
}

mkfn!{
    utf8_char_width_introspect!();
    # [doc = " Given a first byte, determines how many bytes are in this UTF-8 character."] # [unstable (feature = "str_internals" , issue = "none")] # [must_use] # [inline] pub const fn utf8_char_width (b : u8) -> usize { UTF8_CHAR_WIDTH [b as usize] as usize }
}
mkitem!{# [doc = " Mask of the value bits of a continuation byte."] const CONT_MASK : u8 = 0b0011_1111 ;}