macro_rules! deps {
    () => {
        Variant!();
        Field!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl Variant < '_ > { pub (crate) fn from_field (& self) -> Option < & Field > { from_field (& self . fields) } pub (crate) fn source_field (& self) -> Option < & Field > { source_field (& self . fields) } pub (crate) fn backtrace_field (& self) -> Option < & Field > { backtrace_field (& self . fields) } pub (crate) fn distinct_backtrace_field (& self) -> Option < & Field > { let backtrace_field = self . backtrace_field () ? ; distinct_backtrace_field (backtrace_field , self . from_field ()) } }
    };
}

impl_60!()