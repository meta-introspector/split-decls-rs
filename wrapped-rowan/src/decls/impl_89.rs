macro_rules! deps {
    () => {
        SyntaxText!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl From < SyntaxText > for String { fn from (text : SyntaxText) -> String { text . to_string () } }
    };
}

impl_89!()