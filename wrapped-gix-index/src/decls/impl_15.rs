macro_rules! deps {
    () => {
        DirEntry!();
        State!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl DirEntry < '_ > { fn path < 'a > (& self , state : & 'a State) -> & 'a BStr { let range = self . entry . path . start .. self . dir_end ; state . path_backing [range] . as_bstr () } }
    };
}

impl_15!()