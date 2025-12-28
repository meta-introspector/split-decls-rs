macro_rules! deps {
    () => {
        GenericArgs!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl GenericArgs { pub fn is_angle_bracketed (& self) -> bool { matches ! (self , AngleBracketed (..)) } pub fn span (& self) -> Span { match self { AngleBracketed (data) => data . span , Parenthesized (data) => data . span , ParenthesizedElided (span) => * span , } } }
    };
}

impl_17!()