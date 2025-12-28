macro_rules! deps {
    () => {
        IdStorage!();
    };
}

macro_rules! impl_1015 {
    () => {
        deps!();
        impl < T , S > Index < usize > for IdStorage < T , S > { type Output = T ; fn index (& self , index : usize) -> & T { self . elements [index] . as_ref () . unwrap () } }
    };
}

impl_1015!();