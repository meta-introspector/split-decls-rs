macro_rules! deps {
    () => {
        WildStr!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl PartialEq < & str > for WildStr < '_ > { fn eq (& self , other : & & str) -> bool { if self . has_meta { meta_cmp (self . line , other) } else { self . line == * other } } }
    };
}

impl_16!();