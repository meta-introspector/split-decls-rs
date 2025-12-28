macro_rules! deps {
    () => {
        Deserializer!();
    };
}

macro_rules! UnitVariantAccess {
    () => {
        deps!();
        struct UnitVariantAccess < 'a , R : 'a > { de : & 'a mut Deserializer < R > , }
    };
}

UnitVariantAccess!();