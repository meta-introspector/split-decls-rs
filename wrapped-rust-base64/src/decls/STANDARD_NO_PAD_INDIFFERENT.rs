macro_rules! deps {
    () => {
        GeneralPurpose!();
    };
}

macro_rules! STANDARD_NO_PAD_INDIFFERENT {
    () => {
        deps!();
        # [doc = " A [`GeneralPurpose`] engine using the [`alphabet::STANDARD`] base64 alphabet and"] # [doc = " [`NO_PAD_INDIFFERENT`] config."] pub const STANDARD_NO_PAD_INDIFFERENT : GeneralPurpose = GeneralPurpose :: new (& alphabet :: STANDARD , NO_PAD_INDIFFERENT) ;
    };
}

STANDARD_NO_PAD_INDIFFERENT!();