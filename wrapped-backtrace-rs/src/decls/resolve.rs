macro_rules! deps {
    () => {
        Frame!();
        Symbol!();
    };
}

macro_rules! resolve {
    () => {
        deps!();
        # [doc = " Resolve an address to a symbol, passing the symbol to the specified"] # [doc = " closure."] # [doc = ""] # [doc = " This function will look up the given address in areas such as the local"] # [doc = " symbol table, dynamic symbol table, or DWARF debug info (depending on the"] # [doc = " activated implementation) to find symbols to yield."] # [doc = ""] # [doc = " The closure may not be called if resolution could not be performed, and it"] # [doc = " also may be called more than once in the case of inlined functions."] # [doc = ""] # [doc = " Symbols yielded represent the execution at the specified `addr`, returning"] # [doc = " file/line pairs for that address (if available)."] # [doc = ""] # [doc = " Note that if you have a `Frame` then it's recommended to use the"] # [doc = " `resolve_frame` function instead of this one."] # [doc = ""] # [doc = " # Required features"] # [doc = ""] # [doc = " This function requires the `std` feature of the `backtrace` crate to be"] # [doc = " enabled, and the `std` feature is enabled by default."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This function strives to never panic, but if the `cb` provided panics then"] # [doc = " some platforms will force a double panic to abort the process. Some"] # [doc = " platforms use a C library which internally uses callbacks which cannot be"] # [doc = " unwound through, so panicking from `cb` may trigger a process abort."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " extern crate backtrace;"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     backtrace::trace(|frame| {"] # [doc = "         let ip = frame.ip();"] # [doc = ""] # [doc = "         backtrace::resolve(ip, |symbol| {"] # [doc = "             // ..."] # [doc = "         });"] # [doc = ""] # [doc = "         false // only look at the top frame"] # [doc = "     });"] # [doc = " }"] # [doc = " ```"] # [cfg (feature = "std")] pub fn resolve < F : FnMut (& Symbol) > (addr : * mut c_void , cb : F) { let _guard = crate :: lock :: lock () ; unsafe { resolve_unsynchronized (addr , cb) } }
    };
}

resolve!();