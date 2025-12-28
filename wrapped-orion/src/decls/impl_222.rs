macro_rules! deps {
    () => {
        Sha3!();
    };
}

macro_rules! impl_222 {
    () => {
        deps!();
        impl < const RATE : usize > Drop for Sha3 < RATE > { fn drop (& mut self) { self . state . iter_mut () . zeroize () ; self . buffer . iter_mut () . zeroize () ; self . leftover . zeroize () ; } }
    };
}

impl_222!();