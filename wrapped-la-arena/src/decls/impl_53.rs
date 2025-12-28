macro_rules! deps {
    () => {
        IdxRange!();
        Arena!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < T > Index < IdxRange < T > > for Arena < T > { type Output = [T] ; fn index (& self , range : IdxRange < T >) -> & [T] { let start = range . range . start as usize ; let end = range . range . end as usize ; & self . data [start .. end] } }
    };
}

impl_53!();