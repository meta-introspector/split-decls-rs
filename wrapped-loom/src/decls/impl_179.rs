macro_rules! deps {
    () => {
        Id!();
        VersionVec!();
    };
}

macro_rules! impl_179 {
    () => {
        deps!();
        impl ops :: Index < thread :: Id > for VersionVec { type Output = u16 ; fn index (& self , index : thread :: Id) -> & u16 { self . versions . index (index . as_usize ()) } }
    };
}

impl_179!()