macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! Empty {
    () => {
        deps!();
        # [doc = " Error: empty slice"] # [doc = ""] # [doc = " This error is returned when [`Choose::new`] is given an empty slice."] # [derive (Debug , Clone , Copy)] pub struct Empty ;
    };
}

Empty!();