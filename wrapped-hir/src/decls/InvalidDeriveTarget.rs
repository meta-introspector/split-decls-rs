macro_rules! InvalidDeriveTarget {
    () => {
        # [derive (Debug)] pub struct InvalidDeriveTarget { pub node : InFile < SyntaxNodePtr > , }
    };
}

InvalidDeriveTarget!()