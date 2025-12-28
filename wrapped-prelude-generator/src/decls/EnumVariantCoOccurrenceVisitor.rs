macro_rules! deps {
    () => {
        EnumLatticeInfo!();
    };
}

macro_rules! EnumVariantCoOccurrenceVisitor {
    () => {
        deps!();
        struct EnumVariantCoOccurrenceVisitor < 'a > { _enum_name : & 'a str , _current_variant_types : BTreeSet < String > , enum_lattice_info : & 'a mut EnumLatticeInfo , }
    };
}

EnumVariantCoOccurrenceVisitor!()