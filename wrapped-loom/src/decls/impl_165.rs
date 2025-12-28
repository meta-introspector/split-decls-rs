macro_rules! deps {
    () => {
        Set!();
        Thread!();
        Id!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        impl ops :: IndexMut < Id > for Set { fn index_mut (& mut self , index : Id) -> & mut Thread { & mut self . threads [index . id] } }
    };
}

impl_165!()