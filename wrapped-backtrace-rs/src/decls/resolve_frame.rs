macro_rules! deps {
    () => {
        Frame!();
        Symbol!();
    };
}

macro_rules! resolve_frame {
    () => {
        deps!();
        # [doc = " Resolve a previously captured frame to a symbol, passing the symbol to the"] # [doc = " specified closure."] # [doc = ""] # [doc = " This function performs the same function as `resolve` except that it takes a"] # [doc = " `Frame` as an argument instead of an address. This can allow some platform"] # [doc = " implementations of backtracing to provide more accurate symbol information"] # [doc = " or information about inline frames for example. It's recommended to use this"] # [doc = " if you can."] # [doc = ""] # [doc = " # Required features"] # [doc = ""] # [doc = " This function requires the `std` feature of the `backtrace` crate to be"] # [doc = " enabled, and the `std` feature is enabled by default."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This function strives to never panic, but if the `cb` provided panics then"] # [doc = " some platforms will force a double panic to abort the process. Some"] # [doc = " platforms use a C library which internally uses callbacks which cannot be"] # [doc = " unwound through, so panicking from `cb` may trigger a process abort."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " extern crate backtrace;"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     backtrace::trace(|frame| {"] # [doc = "         backtrace::resolve_frame(frame, |symbol| {"] # [doc = "             // ..."] # [doc = "         });"] # [doc = ""] # [doc = "         false // only look at the top frame"] # [doc = "     });"] # [doc = " }"] # [doc = " ```"] # [cfg (feature = "std")] pub fn resolve_frame < F : FnMut (& Symbol) > (frame : & Frame , cb : F) { let _guard = crate :: lock :: lock () ; unsafe { resolve_frame_unsynchronized (frame , cb) } }
    };
}

resolve_frame!();