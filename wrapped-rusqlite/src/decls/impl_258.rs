macro_rules! deps {
    () => {
        ChangesetIter!();
    };
}

macro_rules! impl_258 {
    () => {
        deps!();
        impl Drop for ChangesetIter < '_ > { # [inline] fn drop (& mut self) { unsafe { ffi :: sqlite3changeset_finalize (self . it) ; } } }
    };
}

impl_258!()