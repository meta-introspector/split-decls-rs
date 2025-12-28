macro_rules! deps {
    () => {
        HashValue!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl HashValue { fn desired_pos (& self , mask : usize) -> usize { usize :: from (self . 0) & mask } fn probe_distance (& self , mask : usize , current : usize) -> usize { current . wrapping_sub (self . desired_pos (mask)) & mask } }
    };
}

impl_81!();