macro_rules! deps {
    () => {
        BStr!();
        BSTR!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl BStr { pub unsafe fn from_raw (s : BSTR) -> BStr { BStr (s) } pub fn to_osstring (& self) -> OsString { let len = unsafe { SysStringLen (self . 0) } ; let slice = unsafe { from_raw_parts (self . 0 , len as usize) } ; OsStringExt :: from_wide (slice) } }
    };
}

impl_130!();