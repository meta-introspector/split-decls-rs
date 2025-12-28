macro_rules! deps {
    () => {
        DWORD!();
        Iter!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl < 'a > Iterator for Iter < 'a > { type Item = io :: Result < OsString > ; fn next (& mut self) -> Option < io :: Result < OsString > > { self . idx . next () . and_then (| i | unsafe { let mut v = Vec :: with_capacity (256) ; let mut len = v . capacity () as DWORD ; let ret = RegEnumKeyExW (self . key . raw () , i , v . as_mut_ptr () , & mut len , null_mut () , null_mut () , null_mut () , null_mut () ,) ; if ret == ERROR_NO_MORE_ITEMS { None } else if ret != ERROR_SUCCESS { Some (Err (io :: Error :: from_raw_os_error (ret as i32))) } else { v . set_len (len as usize) ; Some (Ok (OsString :: from_wide (& v))) } }) } }
    };
}

impl_102!();