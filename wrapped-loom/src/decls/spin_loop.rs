macro_rules! spin_loop {
    () => {
        # [doc = " Signals the processor that it is entering a busy-wait spin-loop."] # [doc = ""] # [doc = " For loom, this is an alias of [`yield_now`] but is provided as a reflection"] # [doc = " of the [`core::hint::spin_loop`] function. See the [`yield_now`]"] # [doc = " documentation for more information on what effect this has."] # [doc = ""] # [doc = " [`yield_now`]: crate::thread::yield_now"] pub fn spin_loop () { crate :: sync :: atomic :: spin_loop_hint () ; }
    };
}

spin_loop!()