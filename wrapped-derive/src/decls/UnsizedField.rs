macro_rules! deps {
    () => {
        FieldInfo!();
        UnsizedFieldKind!();
    };
}

macro_rules! UnsizedField {
    () => {
        deps!();
        # [derive (Clone , Debug)] struct UnsizedField < 'a > { kind : UnsizedFieldKind < 'a > , field : FieldInfo < 'a > , }
    };
}

UnsizedField!();