macro_rules! Timespec {
    () => {
        # [derive (Default , Debug , Clone , Copy)] # [repr (transparent)] pub struct Timespec (pub (crate) sys :: __kernel_timespec) ;
    };
}

Timespec!();