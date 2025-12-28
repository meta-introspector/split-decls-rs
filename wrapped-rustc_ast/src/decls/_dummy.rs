macro_rules! deps {
    () => {
        DelimSpan!();
        Spacing!();
        TokenStream!();
        Token!();
        Delimiter!();
    };
}

macro_rules! _dummy {
    () => {
        deps!();
        fn _dummy () where Token : sync :: DynSend + sync :: DynSync , Spacing : sync :: DynSend + sync :: DynSync , DelimSpan : sync :: DynSend + sync :: DynSync , Delimiter : sync :: DynSend + sync :: DynSync , TokenStream : sync :: DynSend + sync :: DynSync , { }
    };
}

_dummy!();