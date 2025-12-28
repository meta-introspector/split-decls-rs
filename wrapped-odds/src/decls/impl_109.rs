macro_rules! deps {
    () => {
        StrideMut!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        unsafe impl < 'a , A > Send for StrideMut < 'a , A > where A : Send { }
    };
}

impl_109!();