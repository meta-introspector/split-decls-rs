macro_rules! PanicNonStr {
    () => {
        # [doc = " A call to a `panic()` lang item where the first argument is _not_ a `&str`."] # [derive (Debug)] pub (crate) struct PanicNonStr ;
    };
}

PanicNonStr!()