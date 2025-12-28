macro_rules! deps {
    () => {
        OutlivesConstraint!();
        PoloniusLocationTable!();
        PoloniusFacts!();
        LoweredConstraints!();
    };
}

macro_rules! emit_outlives_facts {
    () => {
        deps!();
        # [doc = " Emit facts about the outlives constraints: the `subset` base relation, i.e. not a transitive"] # [doc = " closure."] fn emit_outlives_facts < 'tcx > (facts : & mut PoloniusFacts , location_table : & PoloniusLocationTable , constraints : & LoweredConstraints < 'tcx > ,) { facts . subset_base . extend (constraints . outlives_constraints . outlives () . iter () . flat_map (| constraint : & OutlivesConstraint < '_ > | { if let Some (from_location) = constraint . locations . from_location () { Either :: Left (iter :: once ((constraint . sup . into () , constraint . sub . into () , location_table . mid_index (from_location) ,))) } else { Either :: Right (location_table . all_points () . map (move | location | { (constraint . sup . into () , constraint . sub . into () , location) }) ,) } } ,)) ; }
    };
}

emit_outlives_facts!();