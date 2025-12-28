macro_rules! deps {
    () => {
        DefiningUse!();
        RegionCtxt!();
    };
}

macro_rules! CollectMemberConstraintsVisitor {
    () => {
        deps!();
        struct CollectMemberConstraintsVisitor < 'a , 'b , 'tcx > { rcx : & 'a RegionCtxt < 'a , 'tcx > , defining_use : & 'b DefiningUse < 'tcx > , member_constraints : & 'a mut FxHashMap < ConstraintSccIndex , Vec < & 'b DefiningUse < 'tcx > > > , }
    };
}

CollectMemberConstraintsVisitor!()