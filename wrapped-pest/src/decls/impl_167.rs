macro_rules! deps {
    () => {
        Stack!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl < T : Clone > Index < Range < usize > > for Stack < T > { type Output = [T] ; fn index (& self , range : Range < usize >) -> & [T] { self . cache . index (range) } }
    };
}

impl_167!();