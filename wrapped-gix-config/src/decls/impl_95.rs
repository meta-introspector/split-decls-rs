macro_rules! deps {
    () => {
        Size!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl AddAssign < usize > for Size { fn add_assign (& mut self , rhs : usize) { self . 0 += rhs ; } }
    };
}

impl_95!();