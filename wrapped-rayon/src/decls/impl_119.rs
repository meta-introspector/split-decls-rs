macro_rules! deps {
    () => {
        Splitter!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl Splitter { # [inline] fn new () -> Splitter { Splitter { splits : crate :: current_num_threads () , } } # [inline] fn try_split (& mut self , stolen : bool) -> bool { let Splitter { splits } = * self ; if stolen { self . splits = Ord :: max (crate :: current_num_threads () , self . splits / 2) ; true } else if splits > 0 { self . splits /= 2 ; true } else { false } } }
    };
}

impl_119!()