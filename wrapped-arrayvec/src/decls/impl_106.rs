macro_rules! deps {
    () => {
        CapacityError!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl < T > CapacityError < T > { # [doc = " Create a new `CapacityError` from `element`."] pub const fn new (element : T) -> CapacityError < T > { CapacityError { element : element } } # [doc = " Extract the overflowing element"] pub fn element (self) -> T { self . element } # [doc = " Convert into a `CapacityError` that does not carry an element."] pub fn simplify (self) -> CapacityError { CapacityError { element : () } } }
    };
}

impl_106!();