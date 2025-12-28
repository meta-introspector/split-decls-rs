macro_rules! deps {
    () => {
        StructLatticeInfo!();
    };
}

macro_rules! StructFieldCoOccurrenceVisitor {
    () => {
        deps!();
        struct StructFieldCoOccurrenceVisitor < 'a > { _struct_name : & 'a str , current_field_accesses : BTreeSet < String > , struct_lattice_info : & 'a mut StructLatticeInfo , }
    };
}

StructFieldCoOccurrenceVisitor!()