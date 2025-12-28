macro_rules! deps {
    () => {
        Find!();
        Header!();
    };
}

macro_rules! FindObjectOrHeader {
    () => {
        deps!();
        # [doc = " A combination of [`Find`] and [`Header`] traits to help with `dyn` trait objects."] pub trait FindObjectOrHeader : Find + Header { }
    };
}

FindObjectOrHeader!();