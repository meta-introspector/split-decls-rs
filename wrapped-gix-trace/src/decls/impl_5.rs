macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl Drop for Span { fn drop (& mut self) { if let Some ((id , dispatch , _meta)) = self . id . take () { dispatch . exit (& id) ; dispatch . try_close (id) ; } } }
    };
}

impl_5!();