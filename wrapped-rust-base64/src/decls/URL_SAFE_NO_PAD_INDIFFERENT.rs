macro_rules! deps {
    () => {
        GeneralPurpose!();
    };
}

macro_rules! URL_SAFE_NO_PAD_INDIFFERENT {
    () => {
        deps!();
        # [doc = " A [`GeneralPurpose`] engine using the [`alphabet::URL_SAFE`] base64 alphabet and"] # [doc = " [`NO_PAD_INDIFFERENT`] config."] pub const URL_SAFE_NO_PAD_INDIFFERENT : GeneralPurpose = GeneralPurpose :: new (& alphabet :: URL_SAFE , NO_PAD_INDIFFERENT) ;
    };
}

URL_SAFE_NO_PAD_INDIFFERENT!()