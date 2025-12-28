macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! pipe {
    () => {
        deps!();
        # [doc = " Create an interprocess channel."] # [doc = ""] # [doc = " See also [pipe(2)](https://pubs.opengroup.org/onlinepubs/9699919799/functions/pipe.html)"] pub fn pipe () -> std :: result :: Result < (std :: os :: fd :: OwnedFd , std :: os :: fd :: OwnedFd) , Error > { let mut fds = mem :: MaybeUninit :: < [std :: os :: fd :: OwnedFd ; 2] > :: uninit () ; let res = unsafe { libc :: pipe (fds . as_mut_ptr () . cast ()) } ; Error :: result (res) ? ; let [read , write] = unsafe { fds . assume_init () } ; Ok ((read , write)) }
    };
}

pipe!()