macro_rules! deps {
    () => {
        Appearances!();
    };
}

macro_rules! AppearancesIter {
    () => {
        deps!();
        struct AppearancesIter < 'a > { appearances : & 'a Appearances , current : Option < AppearanceIndex > , }
    };
}

AppearancesIter!();