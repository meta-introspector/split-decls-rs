macro_rules! deps {
    () => {
        Key!();
        KeyIterator!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl < 'a > KeyIterator < 'a > { pub (crate) fn new (key : & 'a Key) -> Result < Self > { let mut count = 0 ; let mut max_len = 0 ; let result = unsafe { RegQueryInfoKeyW (key . 0 , null_mut () , null_mut () , null_mut () , & mut count , & mut max_len , null_mut () , null_mut () , null_mut () , null_mut () , null_mut () , null_mut () ,) } ; win32_error (result) . map (| _ | Self { key , range : 0 .. count as usize , name : vec ! [0 ; max_len as usize + 1] , }) } }
    };
}

impl_100!();