macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl std :: ops :: AddAssign < usize > for Bytes { fn add_assign (& mut self , x : usize) { self . 0 += x as isize ; } }
    };
}

impl_17!();