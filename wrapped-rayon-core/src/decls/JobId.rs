macro_rules! JobId {
    () => {
        pub (super) type JobId = (* const () , unsafe fn (* const ())) ;
    };
}

JobId!();