macro_rules! deps {
    () => {
        AutoLogDemangle!();
        AutoParseDemangle!();
    };
}

macro_rules! try_begin_demangle_as_inner {
    () => {
        deps!();
        # [doc = " Automatically log start and end demangling in an s-expression format, when"] # [doc = " the `logging` feature is enabled."] macro_rules ! try_begin_demangle_as_inner { ($ production : expr , $ ctx : expr , $ scope : expr) => { { let _log = AutoLogDemangle :: new ($ production , $ ctx , $ scope , true) ; & mut AutoParseDemangle :: new ($ ctx) ? } } ; }
    };
}

try_begin_demangle_as_inner!();