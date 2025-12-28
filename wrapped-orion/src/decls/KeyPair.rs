macro_rules! deps {
    () => {
        DecapsulationKey!();
        EncapsulationKey!();
    };
}

macro_rules! KeyPair {
    () => {
        deps!();
        # [derive (Debug , PartialEq)] # [doc = " A keypair of X-Wing keys."] pub struct KeyPair { ek : EncapsulationKey , dk : DecapsulationKey , }
    };
}

KeyPair!();