macro_rules! deps {
    () => {
        OwnedKey!();
        Iter!();
        Repr!();
        HKEY!();
        RegistryKey!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl RegistryKey { fn raw (& self) -> HKEY { match self . 0 { Repr :: LocalMachine => HKEY_LOCAL_MACHINE , Repr :: Owned (ref val) => val . 0 , } } # [doc = " Open a sub-key of `self`."] pub fn open (& self , key : & OsStr) -> io :: Result < RegistryKey > { let key = key . encode_wide () . chain (Some (0)) . collect :: < Vec < _ > > () ; let mut ret = null_mut () ; let err = unsafe { RegOpenKeyExW (self . raw () , key . as_ptr () , 0 , KEY_READ | KEY_WOW64_32KEY , & mut ret ,) } ; if err == ERROR_SUCCESS { Ok (RegistryKey (Repr :: Owned (OwnedKey (ret)))) } else { Err (io :: Error :: from_raw_os_error (err as i32)) } } pub fn iter (& self) -> Iter < '_ > { Iter { idx : 0 .. , key : self , } } pub fn query_str (& self , name : & str) -> io :: Result < OsString > { let name : & OsStr = name . as_ref () ; let name = name . encode_wide () . chain (Some (0)) . collect :: < Vec < _ > > () ; let mut len = 0 ; let mut kind = 0 ; unsafe { let err = RegQueryValueExW (self . raw () , name . as_ptr () , null_mut () , & mut kind , null_mut () , & mut len ,) ; if err != ERROR_SUCCESS { return Err (io :: Error :: from_raw_os_error (err as i32)) ; } if kind != REG_SZ { return Err (io :: Error :: new (io :: ErrorKind :: Other , "registry key wasn't a string" ,)) ; } assert ! (len % 2 == 0 , "impossible wide string size: {} bytes" , len) ; let vlen = len as usize / 2 ; let mut v = vec ! [0u16 ; vlen] ; let err = RegQueryValueExW (self . raw () , name . as_ptr () , null_mut () , null_mut () , v . as_mut_ptr () as * mut _ , & mut len ,) ; if err != ERROR_SUCCESS { return Err (io :: Error :: from_raw_os_error (err as i32)) ; } assert ! (len % 2 == 0 , "impossible wide string size: {} bytes" , len) ; let actual_len = len as usize / 2 ; assert ! (actual_len <= v . len ()) ; v . truncate (actual_len) ; if ! v . is_empty () && v [v . len () - 1] == 0 { v . pop () ; } Ok (OsString :: from_wide (& v)) } } }
    };
}

impl_100!();