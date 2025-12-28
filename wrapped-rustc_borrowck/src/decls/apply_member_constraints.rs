macro_rules! deps {
    () => {
        DefiningUse!();
        RegionCtxt!();
        CollectMemberConstraintsVisitor!();
    };
}

macro_rules! apply_member_constraints {
    () => {
        deps!();
        pub (super) fn apply_member_constraints < 'tcx > (rcx : & mut RegionCtxt < '_ , 'tcx > , defining_uses : & [DefiningUse < 'tcx >] ,) { let mut member_constraints = Default :: default () ; for defining_use in defining_uses { let mut visitor = CollectMemberConstraintsVisitor { rcx , defining_use , member_constraints : & mut member_constraints , } ; defining_use . hidden_type . ty . visit_with (& mut visitor) ; } debug ! (? member_constraints) ; for scc_a in rcx . constraint_sccs . all_sccs () { debug ! (? scc_a) ; for & scc_b in rcx . constraint_sccs . successors (scc_a) { debug ! (? scc_b) ; rcx . scc_values . add_region (scc_a , scc_b) ; } for defining_use in member_constraints . get (& scc_a) . into_iter () . flatten () { apply_member_constraint (rcx , scc_a , & defining_use . arg_regions) ; } } }
    };
}

apply_member_constraints!()