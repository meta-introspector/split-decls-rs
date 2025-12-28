macro_rules! deps {
    () => {
        TrimmedPatch!();
        SubstitutionHighlight!();
    };
}

macro_rules! SplicedLines {
    () => {
        deps!();
        pub (crate) type SplicedLines < 'a > = (String , Vec < TrimmedPatch < 'a > > , Vec < Vec < SubstitutionHighlight > > ,) ;
    };
}

SplicedLines!();