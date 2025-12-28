macro_rules! deps {
    () => {
        Item!();
    };
}

macro_rules! only_one {
    () => {
        deps!();
        fn only_one < I , T > (mut iter : I) -> Option < T > where I : Iterator < Item = T > , { iter . next () . filter (| _ | iter . next () . is_none ()) }
    };
}

only_one!()