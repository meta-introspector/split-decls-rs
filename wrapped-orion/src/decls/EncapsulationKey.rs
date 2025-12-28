macro_rules! deps {
    () => {
        EncapKey!();
        MlKem1024Internal!();
    };
}

macro_rules! EncapsulationKey {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Clone)] # [doc = " A type to represent the `EncapsulationKey` that ML-KEM-1024 returns."] pub struct EncapsulationKey { pub (crate) value : EncapKey < 4 , 1568 , MlKem1024Internal > , }
    };
}

EncapsulationKey!()