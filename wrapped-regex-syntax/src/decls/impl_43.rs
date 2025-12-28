macro_rules! deps {
    () => {
        Result!();
        ClassFrame!();
        Formatter!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < 'a > core :: fmt :: Debug for ClassFrame < 'a > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { let x = match * self { ClassFrame :: Union { .. } => "Union" , ClassFrame :: Binary { .. } => "Binary" , ClassFrame :: BinaryLHS { .. } => "BinaryLHS" , ClassFrame :: BinaryRHS { .. } => "BinaryRHS" , } ; write ! (f , "{x}") } }
    };
}

impl_43!()