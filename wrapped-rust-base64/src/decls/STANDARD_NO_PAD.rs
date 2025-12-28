macro_rules! deps {
    () => {
        GeneralPurpose!();
    };
}

macro_rules! STANDARD_NO_PAD {
    () => {
        deps!();
        # [doc = " A [`GeneralPurpose`] engine using the [`alphabet::STANDARD`] base64 alphabet and [`NO_PAD`] config."] pub const STANDARD_NO_PAD : GeneralPurpose = GeneralPurpose :: new (& alphabet :: STANDARD , NO_PAD) ;
    };
}

STANDARD_NO_PAD!();