macro_rules! deps {
    () => {
        Symbol!();
        Frame!();
    };
}

macro_rules! trace {
    () => {
        deps!();
        # [doc = " Inspects the current call-stack, passing all active frames into the closure"] # [doc = " provided to calculate a stack trace."] # [doc = ""] # [doc = " This function is the workhorse of this library in calculating the stack"] # [doc = " traces for a program. The given closure `cb` is yielded instances of a"] # [doc = " `Frame` which represent information about that call frame on the stack. The"] # [doc = " closure is yielded frames in a top-down fashion (most recently called"] # [doc = " functions first)."] # [doc = ""] # [doc = " The closure's return value is an indication of whether the backtrace should"] # [doc = " continue. A return value of `false` will terminate the backtrace and return"] # [doc = " immediately."] # [doc = ""] # [doc = " Once a `Frame` is acquired you will likely want to call `backtrace::resolve`"] # [doc = " to convert the `ip` (instruction pointer) or symbol address to a `Symbol`"] # [doc = " through which the name and/or filename/line number can be learned."] # [doc = ""] # [doc = " Note that this is a relatively low-level function and if you'd like to, for"] # [doc = " example, capture a backtrace to be inspected later, then the `Backtrace`"] # [doc = " type may be more appropriate."] # [doc = ""] # [doc = " # Required features"] # [doc = ""] # [doc = " This function requires the `std` feature of the `backtrace` crate to be"] # [doc = " enabled, and the `std` feature is enabled by default."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This function strives to never panic, but if the `cb` provided panics then"] # [doc = " some platforms will force a double panic to abort the process. Some"] # [doc = " platforms use a C library which internally uses callbacks which cannot be"] # [doc = " unwound through, so panicking from `cb` may trigger a process abort."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " extern crate backtrace;"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     backtrace::trace(|frame| {"] # [doc = "         // ..."] # [doc = ""] # [doc = "         true // continue the backtrace"] # [doc = "     });"] # [doc = " }"] # [doc = " ```"] # [cfg (feature = "std")] pub fn trace < F : FnMut (& Frame) -> bool > (cb : F) { let _guard = crate :: lock :: lock () ; unsafe { trace_unsynchronized (cb) } }
    };
}

trace!()