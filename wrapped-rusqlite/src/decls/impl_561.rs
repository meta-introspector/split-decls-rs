macro_rules! deps {
    () => {
        OrderBy!();
        Column!();
    };
}

macro_rules! impl_561 {
    () => {
        deps!();
        impl OrderBy < '_ > { # [doc = " Column number"] # [inline] # [must_use] pub fn column (& self) -> c_int { self . 0 . iColumn } # [doc = " True for DESC.  False for ASC."] # [inline] # [must_use] pub fn is_order_by_desc (& self) -> bool { self . 0 . desc != 0 } }
    };
}

impl_561!();