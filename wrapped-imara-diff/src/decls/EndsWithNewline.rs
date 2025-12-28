macro_rules! EndsWithNewline {
    () => {
        pub trait EndsWithNewline { fn ends_with_newline (& self) -> bool ; }
    };
}

EndsWithNewline!()