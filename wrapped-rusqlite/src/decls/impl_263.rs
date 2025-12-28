macro_rules! deps {
    () => {
        Changegroup!();
    };
}

macro_rules! impl_263 {
    () => {
        deps!();
        impl Drop for Changegroup { # [inline] fn drop (& mut self) { unsafe { ffi :: sqlite3changegroup_delete (self . cg) ; } } }
    };
}

impl_263!()