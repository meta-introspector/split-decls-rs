macro_rules! deps {
    () => {
        TreeEntry!();
    };
}

macro_rules! FilterCb {
    () => {
        deps!();
        type FilterCb < 'a > = dyn FnMut (& TreeEntry < '_ >) -> bool + 'a ;
    };
}

FilterCb!()