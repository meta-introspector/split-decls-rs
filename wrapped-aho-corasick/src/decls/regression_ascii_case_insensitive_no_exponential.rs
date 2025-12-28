macro_rules! deps {
    () => {
        AhoCorasick!();
    };
}

macro_rules! regression_ascii_case_insensitive_no_exponential {
    () => {
        deps!();
        # [test] fn regression_ascii_case_insensitive_no_exponential () { let ac = AhoCorasick :: builder () . ascii_case_insensitive (true) . build (& ["Tsubaki House-Triple Shot Vol01校花三姐妹"]) . unwrap () ; assert ! (ac . find ("") . is_none ()) ; }
    };
}

regression_ascii_case_insensitive_no_exponential!()