macro_rules! deps {
    () => {
        SyntaxToken!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl PartialEq for SyntaxToken { # [inline] fn eq (& self , other : & SyntaxToken) -> bool { self . data () . key () == other . data () . key () } }
    };
}

impl_29!();