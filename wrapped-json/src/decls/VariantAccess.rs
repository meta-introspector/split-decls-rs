macro_rules! deps {
    () => {
        Deserializer!();
    };
}

macro_rules! VariantAccess {
    () => {
        deps!();
        struct VariantAccess < 'a , R : 'a > { de : & 'a mut Deserializer < R > , }
    };
}

VariantAccess!();