macro_rules! OnInformationalClosure {
    () => {
        struct OnInformationalClosure < F > (F) ;
    };
}

OnInformationalClosure!();