macro_rules! deps {
    () => {
        SubstitutionHighlight!();
        TrimmedPatch!();
    };
}

macro_rules! SplicedLines {
    () => {
        deps!();
        pub (crate) type SplicedLines < 'a > = (String , Vec < TrimmedPatch < 'a > > , Vec < Vec < SubstitutionHighlight > > ,) ;
    };
}

SplicedLines!()