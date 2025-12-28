macro_rules! deps {
    () => {
        TypeInfo!();
        Type!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < 'db > TypeInfo < 'db > { pub fn original (self) -> Type < 'db > { self . original } pub fn has_adjustment (& self) -> bool { self . adjusted . is_some () } # [doc = " The adjusted type, or the original in case no adjustments occurred."] pub fn adjusted (self) -> Type < 'db > { self . adjusted . unwrap_or (self . original) } }
    };
}

impl_79!()