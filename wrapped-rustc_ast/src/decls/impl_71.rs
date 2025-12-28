macro_rules! deps {
    () => {
        UnOp!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl UnOp { pub fn as_str (& self) -> & 'static str { match self { UnOp :: Deref => "*" , UnOp :: Not => "!" , UnOp :: Neg => "-" , } } # [doc = " Returns `true` if the unary operator takes its argument by value."] pub fn is_by_value (self) -> bool { matches ! (self , Self :: Neg | Self :: Not) } }
    };
}

impl_71!();