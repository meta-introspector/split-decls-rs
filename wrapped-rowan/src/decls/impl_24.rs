macro_rules! deps {
    () => {
        SyntaxNode!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl PartialEq for SyntaxNode { # [inline] fn eq (& self , other : & SyntaxNode) -> bool { self . data () . key () == other . data () . key () } }
    };
}

impl_24!()