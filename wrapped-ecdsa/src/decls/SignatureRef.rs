macro_rules! SignatureRef {
    () => {
        struct SignatureRef < 'a > { pub r : UintRef < 'a > , pub s : UintRef < 'a > , }
    };
}

SignatureRef!();