macro_rules! deps {
    () => {
        ErrorVTable!();
        Backtrace!();
    };
}

macro_rules! ErrorImpl {
    () => {
        deps!();
        # [repr (C)] pub (crate) struct ErrorImpl < E = () > { vtable : & 'static ErrorVTable , backtrace : Option < Backtrace > , _object : E , }
    };
}

ErrorImpl!()