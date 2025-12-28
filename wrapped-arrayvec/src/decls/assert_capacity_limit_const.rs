macro_rules! deps {
    () => {
        LenUint!();
    };
}

macro_rules! assert_capacity_limit_const {
    () => {
        deps!();
        macro_rules ! assert_capacity_limit_const { ($ cap : expr) => { if std :: mem :: size_of ::< usize > () > std :: mem :: size_of ::< LenUint > () { if $ cap > LenUint :: MAX as usize { [] [$ cap] } } } }
    };
}

assert_capacity_limit_const!();