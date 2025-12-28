macro_rules! deps {
    () => {
        UnitRef!();
        DwoUnit!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl < R : gimli :: Reader > DwoUnit < R > { fn unit_ref (& self) -> gimli :: UnitRef < '_ , R > { gimli :: UnitRef :: new (& self . sections , & self . dw_unit) } }
    };
}

impl_83!();