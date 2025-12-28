macro_rules! deps {
    () => {
        RemoteHandle!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl < T > RemoteHandle < T > { # [doc = " Drops this handle *without* canceling the underlying future."] # [doc = ""] # [doc = " This method can be used if you want to drop the handle, but let the"] # [doc = " execution continue."] pub fn forget (self) { self . keep_running . store (true , Ordering :: SeqCst) ; } }
    };
}

impl_71!()