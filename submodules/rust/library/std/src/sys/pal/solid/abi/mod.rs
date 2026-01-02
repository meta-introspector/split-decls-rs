mkuse!{use crate :: os :: raw :: c_int ;}
mkmod!{fs, { 
                getname!(fs);
                getsrc!(fs);
                getpath!(fs);
                get_deps!(fs);
                get_crates!(fs);
                mkinclude!(fs);
                 
            }}
mkmod!{sockets, { 
                getname!(sockets);
                getsrc!(sockets);
                getpath!(sockets);
                get_deps!(sockets);
                get_crates!(sockets);
                mkinclude!(sockets);
                 
            }}
mkuse!{pub use self :: fs :: * ;}
mkuse!{pub use super :: itron :: abi :: { E_TMOUT , ER , ER_ID , ID } ;}
mkitem!{pub const SOLID_ERR_NOTFOUND : ER = - 1000 ;}
mkitem!{pub const SOLID_ERR_NOTSUPPORTED : ER = - 1001 ;}
mkitem!{pub const SOLID_ERR_EBADF : ER = - 1002 ;}
mkitem!{pub const SOLID_ERR_INVALIDCONTENT : ER = - 1003 ;}
mkitem!{pub const SOLID_ERR_NOTUSED : ER = - 1004 ;}
mkitem!{pub const SOLID_ERR_ALREADYUSED : ER = - 1005 ;}
mkitem!{pub const SOLID_ERR_OUTOFBOUND : ER = - 1006 ;}
mkitem!{pub const SOLID_ERR_BADSEQUENCE : ER = - 1007 ;}
mkitem!{pub const SOLID_ERR_UNKNOWNDEVICE : ER = - 1008 ;}
mkitem!{pub const SOLID_ERR_BUSY : ER = - 1009 ;}
mkitem!{pub const SOLID_ERR_TIMEOUT : ER = - 1010 ;}
mkitem!{pub const SOLID_ERR_INVALIDACCESS : ER = - 1011 ;}
mkitem!{pub const SOLID_ERR_NOTREADY : ER = - 1012 ;}
mkitem!{mkstruct!{# [repr (C)] # [derive (Debug , Copy , Clone)] pub struct SOLID_RTC_TIME { pub tm_sec : c_int , pub tm_min : c_int , pub tm_hour : c_int , pub tm_mday : c_int , pub tm_mon : c_int , pub tm_year : c_int , pub tm_wday : c_int , }}}
mkitem!{unsafe extern "C" { pub fn SOLID_RTC_ReadTime (time : * mut SOLID_RTC_TIME) -> c_int ; }}
mkitem!{unsafe extern "C" { pub fn SOLID_LOG_write (s : * const u8 , l : usize) ; }}
mkitem!{unsafe extern "C" { pub fn SOLID_TLS_AddDestructor (id : i32 , dtor : unsafe extern "C" fn (* mut u8)) ; }}
mkitem!{unsafe extern "C" { pub fn SOLID_RNG_SampleRandomBytes (buffer : * mut u8 , length : usize) -> c_int ; }}
mkitem!{unsafe extern "C" { pub fn rwl_loc_rdl (id : ID) -> ER ; pub fn rwl_loc_wrl (id : ID) -> ER ; pub fn rwl_ploc_rdl (id : ID) -> ER ; pub fn rwl_ploc_wrl (id : ID) -> ER ; pub fn rwl_unl_rwl (id : ID) -> ER ; pub fn rwl_acre_rwl () -> ER_ID ; pub fn rwl_del_rwl (id : ID) -> ER ; }}