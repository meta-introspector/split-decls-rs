macro_rules! deps {
    () => {
        Shake!();
    };
}

macro_rules! impl_226 {
    () => {
        deps!();
        impl < const RATE : usize > Drop for Shake < RATE > { fn drop (& mut self) { self . state . iter_mut () . zeroize () ; self . buffer . iter_mut () . zeroize () ; self . until_absorb . zeroize () ; self . to_squeeze . zeroize () ; } }
    };
}

impl_226!();