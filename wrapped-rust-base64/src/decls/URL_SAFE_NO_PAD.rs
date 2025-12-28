macro_rules! deps {
    () => {
        GeneralPurpose!();
    };
}

macro_rules! URL_SAFE_NO_PAD {
    () => {
        deps!();
        # [doc = " A [`GeneralPurpose`] engine using the [`alphabet::URL_SAFE`] base64 alphabet and [`NO_PAD`] config."] pub const URL_SAFE_NO_PAD : GeneralPurpose = GeneralPurpose :: new (& alphabet :: URL_SAFE , NO_PAD) ;
    };
}

URL_SAFE_NO_PAD!()