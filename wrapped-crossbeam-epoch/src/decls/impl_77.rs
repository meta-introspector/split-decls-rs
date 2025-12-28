macro_rules! deps {
    () => {
        LocalHandle!();
        Local!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl Drop for LocalHandle { # [inline] fn drop (& mut self) { unsafe { Local :: release_handle (& * self . local) ; } } }
    };
}

impl_77!()