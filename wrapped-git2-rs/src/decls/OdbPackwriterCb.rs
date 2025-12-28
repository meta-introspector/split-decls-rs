macro_rules! deps {
    () => {
        IndexerProgress!();
    };
}

macro_rules! OdbPackwriterCb {
    () => {
        deps!();
        pub (crate) struct OdbPackwriterCb < 'repo > { pub (crate) cb : Option < Box < IndexerProgress < 'repo > > > , }
    };
}

OdbPackwriterCb!()