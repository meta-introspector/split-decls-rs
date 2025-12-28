macro_rules! deps {
    () => {
        MessageTrailers!();
        Binding!();
    };
}

macro_rules! impl_449 {
    () => {
        deps!();
        impl Binding for MessageTrailers { type Raw = * mut raw :: git_message_trailer_array ; unsafe fn from_raw (raw : * mut raw :: git_message_trailer_array) -> MessageTrailers { MessageTrailers { raw : * raw } } fn raw (& self) -> * mut raw :: git_message_trailer_array { & self . raw as * const _ as * mut _ } }
    };
}

impl_449!()