macro_rules! deps {
    () => {
        Timespec!();
    };
}

macro_rules! SubmitArgs {
    () => {
        deps!();
        # [doc = " Submit arguments"] # [doc = ""] # [doc = " Note that arguments that exceed their lifetime will fail to compile."] # [doc = ""] # [doc = " ```compile_fail"] # [doc = " use io_uring::types::{ SubmitArgs, Timespec };"] # [doc = ""] # [doc = " let sigmask: libc::sigset_t = unsafe { std::mem::zeroed() };"] # [doc = ""] # [doc = " let mut args = SubmitArgs::new();"] # [doc = ""] # [doc = " {"] # [doc = "     let ts = Timespec::new();"] # [doc = "     args = args.timespec(&ts);"] # [doc = "     args = args.sigmask(&sigmask);"] # [doc = " }"] # [doc = ""] # [doc = " drop(args);"] # [doc = " ```"] # [repr (transparent)] # [derive (Default , Debug , Clone , Copy)] pub struct SubmitArgs < 'prev : 'now , 'now > { pub (crate) args : sys :: io_uring_getevents_arg , prev : PhantomData < & 'prev () > , now : PhantomData < & 'now () > , }
    };
}

SubmitArgs!()