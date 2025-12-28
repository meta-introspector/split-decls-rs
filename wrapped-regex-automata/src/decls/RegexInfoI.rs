macro_rules! deps {
    () => {
        Config!();
    };
}

macro_rules! RegexInfoI {
    () => {
        deps!();
        # [derive (Clone , Debug)] struct RegexInfoI { config : Config , props : Vec < hir :: Properties > , props_union : hir :: Properties , }
    };
}

RegexInfoI!();