macro_rules! deps {
    () => {
        ModuleBuffer!();
    };
}

macro_rules! other_482 {
    () => {
        deps!();
        unsafe extern "C" { pub (crate) type ModuleBuffer ; }
    };
}

other_482!()