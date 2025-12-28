macro_rules! deps {
    () => {
        Reference!();
        Platform!();
    };
}

macro_rules! impl_274 {
    () => {
        deps!();
        impl Reference < '_ > { # [doc = " Return a platform for obtaining iterators over reference logs."] pub fn log_iter (& self) -> gix_ref :: file :: log :: iter :: Platform < '_ , '_ > { self . inner . log_iter (& self . repo . refs) } # [doc = " Return true if a reflog is present for this reference."] pub fn log_exists (& self) -> bool { self . inner . log_exists (& self . repo . refs) } }
    };
}

impl_274!()