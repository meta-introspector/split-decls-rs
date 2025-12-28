macro_rules! deps {
    () => {
        GeneralPurpose!();
    };
}

macro_rules! URL_SAFE_ENGINE {
    () => {
        deps!();
        const URL_SAFE_ENGINE : GeneralPurpose = GeneralPurpose :: new (& URL_SAFE , PAD) ;
    };
}

URL_SAFE_ENGINE!()