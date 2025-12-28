macro_rules! deps {
    () => {
        Enter!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl Drop for Enter { fn drop (& mut self) { ENTERED . with (| c | { assert ! (c . get ()) ; c . set (false) ; }) ; } }
    };
}

impl_60!();