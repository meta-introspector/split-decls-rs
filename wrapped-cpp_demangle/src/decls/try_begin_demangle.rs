macro_rules! deps {
    () => {
        AutoParseDemangle!();
        AutoLogDemangle!();
    };
}

macro_rules! try_begin_demangle {
    () => {
        deps!();
        # [doc = " Automatically log start and end demangling in an s-expression format, when"] # [doc = " the `logging` feature is enabled."] macro_rules ! try_begin_demangle { ($ production : expr , $ ctx : expr , $ scope : expr) => { { let _log = AutoLogDemangle :: new ($ production , $ ctx , $ scope , false) ; & mut AutoParseDemangle :: new ($ ctx) ? } } ; }
    };
}

try_begin_demangle!()