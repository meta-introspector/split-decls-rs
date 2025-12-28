macro_rules! AbiFromStrErr {
    () => {
        # [derive (Clone , Debug)] pub enum AbiFromStrErr { # [doc = " not a known ABI"] Unknown , # [doc = " no \"-unwind\" variant can be used here"] NoExplicitUnwind , }
    };
}

AbiFromStrErr!()