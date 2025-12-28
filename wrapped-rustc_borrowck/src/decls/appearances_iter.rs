macro_rules! deps {
    () => {
        AppearancesIter!();
        Appearances!();
    };
}

macro_rules! appearances_iter {
    () => {
        deps!();
        fn appearances_iter (first : Option < AppearanceIndex > , appearances : & Appearances ,) -> impl Iterator < Item = AppearanceIndex > { AppearancesIter { appearances , current : first } }
    };
}

appearances_iter!();