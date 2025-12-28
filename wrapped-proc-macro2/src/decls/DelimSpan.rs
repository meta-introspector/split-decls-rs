macro_rules! deps {
    () => {
        DelimSpanEnum!();
        Group!();
        ProcMacroAutoTraits!();
    };
}

macro_rules! DelimSpan {
    () => {
        deps!();
        # [doc = " An object that holds a [`Group`]'s `span_open()` and `span_close()` together"] # [doc = " in a more compact representation than holding those 2 spans individually."] # [doc = ""] # [doc = " [`Group`]: crate::Group"] # [derive (Copy , Clone)] pub struct DelimSpan { inner : DelimSpanEnum , _marker : ProcMacroAutoTraits , }
    };
}

DelimSpan!()