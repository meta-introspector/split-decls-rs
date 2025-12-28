macro_rules! deps {
    () => {
        Id!();
        Thread!();
        Set!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        impl ops :: Index < Id > for Set { type Output = Thread ; fn index (& self , index : Id) -> & Thread { & self . threads [index . id] } }
    };
}

impl_164!();