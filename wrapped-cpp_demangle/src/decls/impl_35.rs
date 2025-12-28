macro_rules! deps {
    () => {
        Result!();
        DemangleContext!();
        Error!();
        DemangleWrite!();
        AutoParseDemangle!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < 'a , 'b , W : 'a + DemangleWrite > AutoParseDemangle < 'a , 'b , W > { # [inline] fn new (ctx : & 'b mut DemangleContext < 'a , W >) -> core :: result :: Result < Self , fmt :: Error > { ctx . enter_recursion () ? ; Ok (AutoParseDemangle (ctx)) } }
    };
}

impl_35!();