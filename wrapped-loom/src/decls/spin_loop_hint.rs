macro_rules! spin_loop_hint {
    () => {
        # [doc = " Signals the processor that it is entering a busy-wait spin-loop."] # [doc = ""] # [doc = " For loom, this is an alias of [`yield_now`] but is provided as a reflection"] # [doc = " of the deprecated [`core::sync::atomic::spin_loop_hint`] function. See the"] # [doc = " [`yield_now`] documentation for more information on what effect using this"] # [doc = " has on loom."] # [doc = ""] # [doc = " [`yield_now`]: crate::thread::yield_now"] pub fn spin_loop_hint () { crate :: thread :: yield_now () ; }
    };
}

spin_loop_hint!();