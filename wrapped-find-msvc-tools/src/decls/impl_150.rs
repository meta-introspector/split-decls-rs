macro_rules! deps {
    () => {
        SetupInstance!();
        ComPtr!();
        BStr!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl SetupInstance { pub unsafe fn from_raw (obj : * mut ISetupInstance) -> SetupInstance { SetupInstance (ComPtr :: from_raw (obj)) } pub fn instance_id (& self) -> Result < OsString , i32 > { let mut s = null () ; let err = unsafe { self . 0 . GetInstanceId (& mut s) } ; let bstr = unsafe { BStr :: from_raw (s) } ; if err < 0 { return Err (err) ; } Ok (bstr . to_osstring ()) } pub fn installation_name (& self) -> Result < OsString , i32 > { let mut s = null () ; let err = unsafe { self . 0 . GetInstallationName (& mut s) } ; let bstr = unsafe { BStr :: from_raw (s) } ; if err < 0 { return Err (err) ; } Ok (bstr . to_osstring ()) } pub fn installation_path (& self) -> Result < OsString , i32 > { let mut s = null () ; let err = unsafe { self . 0 . GetInstallationPath (& mut s) } ; let bstr = unsafe { BStr :: from_raw (s) } ; if err < 0 { return Err (err) ; } Ok (bstr . to_osstring ()) } pub fn installation_version (& self) -> Result < OsString , i32 > { let mut s = null () ; let err = unsafe { self . 0 . GetInstallationVersion (& mut s) } ; let bstr = unsafe { BStr :: from_raw (s) } ; if err < 0 { return Err (err) ; } Ok (bstr . to_osstring ()) } pub fn product_path (& self) -> Result < OsString , i32 > { let mut s = null () ; let this = self . 0 . cast :: < ISetupInstance2 > () ? ; let err = unsafe { this . GetProductPath (& mut s) } ; let bstr = unsafe { BStr :: from_raw (s) } ; if err < 0 { return Err (err) ; } Ok (bstr . to_osstring ()) } }
    };
}

impl_150!();