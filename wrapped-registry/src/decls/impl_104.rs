macro_rules! deps {
    () => {
        Key!();
        ValueIterator!();
        Data!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl < 'a > ValueIterator < 'a > { pub (crate) fn new (key : & 'a Key) -> Result < Self > { let mut count = 0 ; let mut name_max_len = 0 ; let mut value_max_len = 0 ; let result = unsafe { RegQueryInfoKeyW (key . 0 , null_mut () , null_mut () , null_mut () , null_mut () , null_mut () , null_mut () , & mut count , & mut name_max_len , & mut value_max_len , null_mut () , null_mut () ,) } ; win32_error (result) ? ; Ok (Self { key , range : 0 .. count as usize , name : vec ! [0 ; name_max_len as usize + 1] , data : Data :: new (value_max_len as usize) , }) } }
    };
}

impl_104!();