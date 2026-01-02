mkmod!{unsupported_env, { 
                getname!(unsupported_env);
                getsrc!(unsupported_env);
                getpath!(unsupported_env);
                get_deps!(unsupported_env);
                get_crates!(unsupported_env);
                mkinclude!(unsupported_env);
                 
            }}
mkuse!{pub use unsupported_env :: { Env , env , setenv , unsetenv } ;}
mkuse!{use crate :: ffi :: { OsStr , OsString } ;}
mkuse!{use crate :: sys :: os_str ;}
mkuse!{use crate :: sys :: pal :: { WORD_SIZE , abi } ;}
mkuse!{use crate :: sys_common :: FromInner ;}

macro_rules! getenv_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function getenv in module {}", module_path!());
    };
}

mkfn!{
    getenv_introspect!();
    pub fn getenv (varname : & OsStr) -> Option < OsString > { let varname = varname . as_encoded_bytes () ; let nbytes = unsafe { abi :: sys_getenv (crate :: ptr :: null_mut () , 0 , varname . as_ptr () , varname . len ()) } ; if nbytes == usize :: MAX { return None ; } let nwords = (nbytes + WORD_SIZE - 1) / WORD_SIZE ; let words = unsafe { abi :: sys_alloc_words (nwords) } ; let nbytes2 = unsafe { abi :: sys_getenv (words , nwords , varname . as_ptr () , varname . len ()) } ; debug_assert_eq ! (nbytes , nbytes2) ; let u8s : & [u8] = unsafe { crate :: slice :: from_raw_parts (words . cast () as * const u8 , nbytes) } ; Some (OsString :: from_inner (os_str :: Buf { inner : u8s . to_vec () })) }
}