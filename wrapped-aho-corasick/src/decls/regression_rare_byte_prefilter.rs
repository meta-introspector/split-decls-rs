macro_rules! deps {
    () => {
        AhoCorasick!();
    };
}

macro_rules! regression_rare_byte_prefilter {
    () => {
        deps!();
        # [test] fn regression_rare_byte_prefilter () { use crate :: AhoCorasick ; let ac = AhoCorasick :: new (& ["ab/j/" , "x/"]) . unwrap () ; assert ! (ac . is_match ("ab/j/")) ; }
    };
}

regression_rare_byte_prefilter!();