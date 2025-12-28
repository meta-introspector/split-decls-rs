macro_rules! deps {
    () => {
        SpanGuard!();
    };
}

macro_rules! impl_313 {
    () => {
        deps!();
        impl Drop for SpanGuard { fn drop (& mut self) { self . 0 . with_subscriber (| (id , dispatch) | { dispatch . exit (id) ; }) ; } }
    };
}

impl_313!()