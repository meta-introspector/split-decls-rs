macro_rules! deps {
    () => {
        GateIssue!();
    };
}

macro_rules! find_feature_issue {
    () => {
        deps!();
        pub fn find_feature_issue (feature : Symbol , issue : GateIssue) -> Option < NonZero < u32 > > { match issue { GateIssue :: Language => find_lang_feature_issue (feature) , GateIssue :: Library (lib) => lib , } }
    };
}

find_feature_issue!();