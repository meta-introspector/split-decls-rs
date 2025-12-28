macro_rules! IntoStream {
    () => {
        pub trait IntoStream { fn into_stream (self) -> Vec < u8 > ; }
    };
}

IntoStream!()