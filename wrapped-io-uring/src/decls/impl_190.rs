macro_rules! deps {
    () => {
        Timespec!();
        SubmitArgs!();
    };
}

macro_rules! impl_190 {
    () => {
        deps!();
        impl < 'prev , 'now > SubmitArgs < 'prev , 'now > { # [inline] pub const fn new () -> SubmitArgs < 'static , 'static > { let args = sys :: io_uring_getevents_arg { sigmask : 0 , sigmask_sz : 0 , min_wait_usec : 0 , ts : 0 , } ; SubmitArgs { args , prev : PhantomData , now : PhantomData , } } # [inline] # [doc = " Signals to mask during waiting for the result"] # [doc = ""] # [doc = " Masked signals will be restored after submit operation returns"] pub fn sigmask < 'new > (mut self , sigmask : & 'new libc :: sigset_t) -> SubmitArgs < 'now , 'new > { self . args . sigmask = cast_ptr (sigmask) as _ ; self . args . sigmask_sz = std :: mem :: size_of :: < libc :: sigset_t > () as _ ; SubmitArgs { args : self . args , prev : self . now , now : PhantomData , } } # [doc = " Sets a timeout in microseconds to start waiting for a minimum of a single completion."] # [doc = ""] # [doc = " Once the timeout expires, the kernel will return when a single completion has been received"] # [doc = " instead of waiting for the minimum amount of completions specified by the `want` parameter"] # [doc = " in the call to [`Submitter::submit_and_wait`](crate::Submitter::submit_and_wait) or"] # [doc = " [`Submitter::submit_with_args`](crate::Submitter::submit_with_args)."] # [doc = ""] # [doc = " Available since 6.12. Use the"] # [doc = " [`Parameters::is_feature_min_timeout`](crate::Parameters::is_feature_min_timeout) method to"] # [doc = " check for availability."] # [inline] pub fn min_wait_usec (mut self , min_wait_usec : u32) -> Self { self . args . min_wait_usec = min_wait_usec ; self } # [inline] # [doc = " Timeout for submit operation"] pub fn timespec < 'new > (mut self , timespec : & 'new Timespec) -> SubmitArgs < 'now , 'new > { self . args . ts = cast_ptr (timespec) as _ ; SubmitArgs { args : self . args , prev : self . now , now : PhantomData , } } }
    };
}

impl_190!();