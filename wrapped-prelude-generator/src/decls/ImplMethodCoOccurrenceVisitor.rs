macro_rules! deps {
    () => {
        ImplLatticeInfo!();
    };
}

macro_rules! ImplMethodCoOccurrenceVisitor {
    () => {
        deps!();
        struct ImplMethodCoOccurrenceVisitor < 'a > { _impl_for_type : & 'a str , current_method_calls : BTreeSet < String > , impl_lattice_info : & 'a mut ImplLatticeInfo , }
    };
}

ImplMethodCoOccurrenceVisitor!();