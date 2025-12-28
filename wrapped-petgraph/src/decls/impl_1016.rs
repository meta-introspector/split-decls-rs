macro_rules! deps {
    () => {
        IdStorage!();
    };
}

macro_rules! impl_1016 {
    () => {
        deps!();
        impl < T , S > IndexMut < usize > for IdStorage < T , S > { fn index_mut (& mut self , index : usize) -> & mut T { self . elements [index] . as_mut () . unwrap () } }
    };
}

impl_1016!();