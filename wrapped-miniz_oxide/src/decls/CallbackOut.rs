macro_rules! deps {
    () => {
        CallbackFunc!();
        CallbackBuf!();
    };
}

macro_rules! CallbackOut {
    () => {
        deps!();
        enum CallbackOut < 'a > { Func (CallbackFunc < 'a >) , Buf (CallbackBuf < 'a >) , }
    };
}

CallbackOut!();