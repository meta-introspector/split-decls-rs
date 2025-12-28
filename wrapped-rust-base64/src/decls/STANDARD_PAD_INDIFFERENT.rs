macro_rules! deps {
    () => {
        GeneralPurpose!();
    };
}

macro_rules! STANDARD_PAD_INDIFFERENT {
    () => {
        deps!();
        # [doc = " A [`GeneralPurpose`] engine using the [`alphabet::STANDARD`] base64 alphabet and"] # [doc = " [`PAD_INDIFFERENT`] config."] pub const STANDARD_PAD_INDIFFERENT : GeneralPurpose = GeneralPurpose :: new (& alphabet :: STANDARD , PAD_INDIFFERENT) ;
    };
}

STANDARD_PAD_INDIFFERENT!()