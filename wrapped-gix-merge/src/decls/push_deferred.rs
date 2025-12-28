macro_rules! deps {
    () => {
        ChangeList!();
    };
}

macro_rules! push_deferred {
    () => {
        deps!();
        fn push_deferred (change_and_idx : (Change , Option < usize >) , changes : & mut ChangeList) { push_deferred_with_rewrite (change_and_idx , None , changes) ; }
    };
}

push_deferred!();