macro_rules! deps {
    () => {
        Size!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl AddAssign for Size { # [inline] fn add_assign (& mut self , other : Size) { * self = * self + other ; } }
    };
}

impl_75!()