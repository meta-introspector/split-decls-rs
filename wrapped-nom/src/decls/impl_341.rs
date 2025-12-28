macro_rules! deps {
    () => {
        FindSubstring!();
        AsBytes!();
    };
}

macro_rules! impl_341 {
    () => {
        deps!();
        impl < 'a , 'b > FindSubstring < & 'b str > for & 'a [u8] { fn find_substring (& self , substr : & 'b str) -> Option < usize > { self . find_substring (AsBytes :: as_bytes (substr)) } }
    };
}

impl_341!();