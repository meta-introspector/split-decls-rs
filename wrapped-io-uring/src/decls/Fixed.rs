macro_rules! deps {
    () => {
        Fd!();
    };
}

macro_rules! Fixed {
    () => {
        deps!();
        # [doc = " A file descriptor that has been registered with io_uring using"] # [doc = " [`Submitter::register_files`](crate::Submitter::register_files) or [`Submitter::register_files_sparse`](crate::Submitter::register_files_sparse)."] # [doc = " This can reduce overhead compared to using [`Fd`] in some cases."] # [derive (Debug , Clone , Copy)] # [repr (transparent)] pub struct Fixed (pub u32) ;
    };
}

Fixed!();