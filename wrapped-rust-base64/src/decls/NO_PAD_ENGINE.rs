macro_rules! deps {
    () => {
        GeneralPurpose!();
    };
}

macro_rules! NO_PAD_ENGINE {
    () => {
        deps!();
        const NO_PAD_ENGINE : GeneralPurpose = GeneralPurpose :: new (& STANDARD , NO_PAD) ;
    };
}

NO_PAD_ENGINE!();