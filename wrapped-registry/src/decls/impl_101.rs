macro_rules! deps {
    () => {
        KeyIterator!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl Iterator for KeyIterator < '_ > { type Item = String ; fn next (& mut self) -> Option < Self :: Item > { self . range . next () . and_then (| index | { let mut len = self . name . len () as u32 ; let result = unsafe { RegEnumKeyExW (self . key . 0 , index as u32 , self . name . as_mut_ptr () , & mut len , null () , null_mut () , null_mut () , null_mut () ,) } ; if result != 0 { debug_assert_eq ! (result , ERROR_NO_MORE_ITEMS) ; None } else { Some (String :: from_utf16_lossy (& self . name [0 .. len as usize])) } }) } }
    };
}

impl_101!();