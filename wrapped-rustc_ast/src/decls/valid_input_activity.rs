macro_rules! deps {
    () => {
        DiffMode!();
        DiffActivity!();
        Const!();
    };
}

macro_rules! valid_input_activity {
    () => {
        deps!();
        pub fn valid_input_activity (mode : DiffMode , activity : DiffActivity) -> bool { use DiffActivity :: * ; return match mode { DiffMode :: Error => false , DiffMode :: Source => false , DiffMode :: Forward => activity . is_dual_or_const () , DiffMode :: Reverse => { matches ! (activity , Active | ActiveOnly | Duplicated | DuplicatedOnly | Const) } } ; }
    };
}

valid_input_activity!();