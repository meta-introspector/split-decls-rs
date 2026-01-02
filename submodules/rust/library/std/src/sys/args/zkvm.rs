mkuse!{use crate :: ffi :: OsString ;}
mkuse!{use crate :: fmt ;}
mkuse!{use crate :: sys :: os_str ;}
mkuse!{use crate :: sys :: pal :: { WORD_SIZE , abi } ;}
mkuse!{use crate :: sys_common :: FromInner ;}
mkitem!{mkstruct!{pub struct Args { i_forward : usize , i_back : usize , count : usize , }}}

macro_rules! args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function args in module {}", module_path!());
    };
}

mkfn!{
    args_introspect!();
    pub fn args () -> Args { let count = unsafe { abi :: sys_argc () } ; Args { i_forward : 0 , i_back : 0 , count } }
}
mkitem!{mkimpl!{impl Args { # [doc = " Use sys_argv to get the arg at the requested index. Does not check that i is less than argc"] # [doc = " and will not return if the index is out of bounds."] fn argv (i : usize) -> OsString { let arg_len = unsafe { abi :: sys_argv (crate :: ptr :: null_mut () , 0 , i) } ; let arg_len_words = (arg_len + WORD_SIZE - 1) / WORD_SIZE ; let words = unsafe { abi :: sys_alloc_words (arg_len_words) } ; let arg_len2 = unsafe { abi :: sys_argv (words , arg_len_words , i) } ; debug_assert_eq ! (arg_len , arg_len2) ; let arg_bytes : & [u8] = unsafe { crate :: slice :: from_raw_parts (words . cast () as * const u8 , arg_len) } ; OsString :: from_inner (os_str :: Buf { inner : arg_bytes . to_vec () }) } }}}
mkitem!{mkimpl!{impl fmt :: Debug for Args { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . finish () } }}}
mkitem!{mkimpl!{impl Iterator for Args { type Item = OsString ; fn next (& mut self) -> Option < OsString > { if self . i_forward >= self . count - self . i_back { None } else { let arg = Self :: argv (self . i_forward) ; self . i_forward += 1 ; Some (arg) } } fn size_hint (& self) -> (usize , Option < usize >) { (self . count , Some (self . count)) } }}}
mkitem!{mkimpl!{impl ExactSizeIterator for Args { fn len (& self) -> usize { self . count } }}}
mkitem!{mkimpl!{impl DoubleEndedIterator for Args { fn next_back (& mut self) -> Option < OsString > { if self . i_back >= self . count - self . i_forward { None } else { let arg = Self :: argv (self . count - 1 - self . i_back) ; self . i_back += 1 ; Some (arg) } } }}}