macro_rules! deps {
    () => {
        Pod!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        unsafe impl < const N : usize , T : Pod > Pod for [T ; N] { }
    };
}

impl_79!();