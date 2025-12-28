macro_rules! deps {
    () => {
        Reader!();
    };
}

macro_rules! impl_457 {
    () => {
        deps!();
        impl Drop for Reader { fn drop (& mut self) { for file in & self . 1 { unsafe { _ = Box :: from_raw (* file) ; } } } }
    };
}

impl_457!();