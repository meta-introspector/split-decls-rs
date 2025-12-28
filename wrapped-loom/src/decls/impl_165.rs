macro_rules! deps {
    () => {
        Thread!();
        Id!();
        Set!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        impl ops :: IndexMut < Id > for Set { fn index_mut (& mut self , index : Id) -> & mut Thread { & mut self . threads [index . id] } }
    };
}

impl_165!();