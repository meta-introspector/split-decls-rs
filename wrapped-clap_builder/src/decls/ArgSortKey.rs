macro_rules! deps {
    () => {
        Arg!();
    };
}

macro_rules! ArgSortKey {
    () => {
        deps!();
        type ArgSortKey = fn (arg : & Arg) -> (usize , String) ;
    };
}

ArgSortKey!();