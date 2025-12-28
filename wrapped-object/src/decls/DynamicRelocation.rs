macro_rules! deps {
    () => {
        Relocation!();
    };
}

macro_rules! DynamicRelocation {
    () => {
        deps!();
        # [doc = " A dynamic relocation."] pub type DynamicRelocation = Relocation < true > ;
    };
}

DynamicRelocation!();