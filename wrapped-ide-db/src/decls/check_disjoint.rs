macro_rules! deps {
    () => {
        Indel!();
    };
}

macro_rules! check_disjoint {
    () => {
        deps!();
        fn check_disjoint < 'a , I > (indels : & mut I) -> bool where I : std :: iter :: Iterator < Item = & 'a Indel > + Clone , { indels . clone () . zip (indels . skip (1)) . all (| (l , r) | l . delete . end () <= r . delete . start () || l == r) }
    };
}

check_disjoint!()