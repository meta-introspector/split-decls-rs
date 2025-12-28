macro_rules! deps {
    () => {
        FindSubstring!();
    };
}

macro_rules! impl_342 {
    () => {
        deps!();
        impl < 'a , 'b > FindSubstring < & 'b str > for & 'a str { fn find_substring (& self , substr : & 'b str) -> Option < usize > { self . find (substr) } }
    };
}

impl_342!()