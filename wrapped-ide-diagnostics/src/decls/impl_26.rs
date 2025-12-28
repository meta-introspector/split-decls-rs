macro_rules! deps {
    () => {
        LintGroups!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl LintGroups { fn contains (& self , group : & str) -> bool { self . groups . contains (& group) || (self . inside_warnings && group == "warnings") } }
    };
}

impl_26!()