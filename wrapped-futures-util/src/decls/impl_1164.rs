macro_rules! deps {
    () => {
        ReadLine!();
    };
}

macro_rules! impl_1164 {
    () => {
        deps!();
        impl < R : ? Sized > Drop for ReadLine < '_ , R > { fn drop (& mut self) { if ! self . finished { self . bytes . truncate (self . bytes . len () - self . read) ; mem :: swap (unsafe { self . buf . as_mut_vec () } , & mut self . bytes) ; } } }
    };
}

impl_1164!()