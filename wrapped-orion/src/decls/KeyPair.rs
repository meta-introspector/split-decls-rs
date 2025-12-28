macro_rules! deps {
    () => {
        EncapsulationKey!();
        DecapsulationKey!();
    };
}

macro_rules! KeyPair {
    () => {
        deps!();
        # [derive (Debug , PartialEq)] # [doc = " A keypair of X-Wing keys."] pub struct KeyPair { ek : EncapsulationKey , dk : DecapsulationKey , }
    };
}

KeyPair!()