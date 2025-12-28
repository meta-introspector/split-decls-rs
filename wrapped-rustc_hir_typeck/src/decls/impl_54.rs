macro_rules! deps {
    () => {
        Diverges!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl Diverges { # [doc = " Creates a `Diverges::Always` with the provided `span` and the default note message."] pub (super) fn always (span : Span) -> Diverges { Diverges :: Always { span , custom_note : None } } pub (super) fn is_always (self) -> bool { self >= Diverges :: Always { span : DUMMY_SP , custom_note : None } } }
    };
}

impl_54!()