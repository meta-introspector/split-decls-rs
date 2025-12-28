macro_rules! deps {
    () => {
        Never!();
        Error!();
        Find!();
        Data!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl super :: Find for Never { fn try_find < 'a > (& self , _id : & gix_hash :: oid , _buffer : & 'a mut Vec < u8 >) -> Result < Option < crate :: Data < 'a > > , Error > { Ok (None) } }
    };
}

impl_162!();