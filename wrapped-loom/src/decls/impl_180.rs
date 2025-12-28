macro_rules! deps {
    () => {
        Id!();
        VersionVec!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl ops :: IndexMut < thread :: Id > for VersionVec { fn index_mut (& mut self , index : thread :: Id) -> & mut u16 { self . versions . index_mut (index . as_usize ()) } }
    };
}

impl_180!()