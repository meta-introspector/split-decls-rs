macro_rules! deps {
    () => {
        GeneralPurpose!();
    };
}

macro_rules! URL_SAFE_PAD_INDIFFERENT {
    () => {
        deps!();
        # [doc = " A [`GeneralPurpose`] engine using the [`alphabet::URL_SAFE`] base64 alphabet and"] # [doc = " [`PAD_INDIFFERENT`] config."] pub const URL_SAFE_PAD_INDIFFERENT : GeneralPurpose = GeneralPurpose :: new (& alphabet :: URL_SAFE , PAD_INDIFFERENT) ;
    };
}

URL_SAFE_PAD_INDIFFERENT!()