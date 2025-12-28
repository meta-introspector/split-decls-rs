macro_rules! deps {
    () => {
        SpanError!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        # [doc = " Manual implementation because [`Span`] is not [`PartialEq`] and can be ignored when comparing."] impl PartialEq for SpanError { fn eq (& self , other : & SpanError) -> bool { self . err == other . err } }
    };
}

impl_71!()