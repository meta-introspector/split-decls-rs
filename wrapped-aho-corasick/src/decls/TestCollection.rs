macro_rules! deps {
    () => {
        SearchTest!();
    };
}

macro_rules! TestCollection {
    () => {
        deps!();
        # [doc = " A collection of test groups."] type TestCollection = & 'static [& 'static [SearchTest]] ;
    };
}

TestCollection!()