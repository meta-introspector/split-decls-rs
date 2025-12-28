macro_rules! deps {
    () => {
        MessageTrailers!();
    };
}

macro_rules! impl_448 {
    () => {
        deps!();
        impl Drop for MessageTrailers { fn drop (& mut self) { unsafe { raw :: git_message_trailer_array_free (& mut self . raw) ; } } }
    };
}

impl_448!();