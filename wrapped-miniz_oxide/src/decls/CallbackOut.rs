macro_rules! deps {
    () => {
        CallbackBuf!();
        CallbackFunc!();
    };
}

macro_rules! CallbackOut {
    () => {
        deps!();
        enum CallbackOut < 'a > { Func (CallbackFunc < 'a >) , Buf (CallbackBuf < 'a >) , }
    };
}

CallbackOut!()