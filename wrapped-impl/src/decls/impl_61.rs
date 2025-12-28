macro_rules! deps {
    () => {
        Field!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl Field < '_ > { pub (crate) fn is_backtrace (& self) -> bool { type_is_backtrace (self . ty) } pub (crate) fn source_span (& self) -> Span { if let Some (source_attr) = & self . attrs . source { source_attr . span } else if let Some (from_attr) = & self . attrs . from { from_attr . span } else { self . member . span () } } }
    };
}

impl_61!()