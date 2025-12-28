macro_rules! deps {
    () => {
        RegexInfoI!();
    };
}

macro_rules! RegexInfo {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub (crate) struct RegexInfo (Arc < RegexInfoI >) ;
    };
}

RegexInfo!();