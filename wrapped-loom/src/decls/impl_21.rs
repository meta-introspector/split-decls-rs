macro_rules! deps {
    () => {
        State!();
        Allocation!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl State { pub (super) fn check_for_leaks (& self , index : usize) { if ! self . is_dropped { if self . allocated . is_captured () { panic ! ("Allocation leaked.\n  Allocated: {}\n      Index: {}" , self . allocated , index) ; } else { panic ! ("Allocation leaked.\n  Index: {}" , index) ; } } } }
    };
}

impl_21!();