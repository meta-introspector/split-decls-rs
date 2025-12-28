macro_rules! deps {
    () => {
        LivenessValues!();
        BorrowSet!();
        LocalizedOutlivesConstraintSet!();
    };
}

macro_rules! emit_mermaid_constraint_graph {
    () => {
        deps!();
        # [doc = " Emits a mermaid flowchart of the polonius localized outlives constraints, with subgraphs per"] # [doc = " region, and loan introductions."] fn emit_mermaid_constraint_graph < 'tcx > (borrow_set : & BorrowSet < 'tcx > , liveness : & LivenessValues , localized_outlives_constraints : & LocalizedOutlivesConstraintSet , out : & mut dyn io :: Write ,) -> io :: Result < usize > { let location_name = | location : Location | { format ! ("BB{}_{}" , location . block . index () , location . statement_index) } ; let region_name = | region : RegionVid | format ! ("'{}" , region . index ()) ; let node_name = | region : RegionVid , point : PointIndex | { let location = liveness . location_from_point (point) ; format ! ("{}_{}" , region_name (region) , location_name (location)) } ; writeln ! (out , "flowchart TD") ? ; writeln ! (out , "    subgraph \"Loans\"") ? ; for loan_idx in 0 .. borrow_set . len () { writeln ! (out , "        L{loan_idx}") ? ; } writeln ! (out , "    end\n") ? ; for (loan_idx , loan) in borrow_set . iter_enumerated () { writeln ! (out , "    L{} --> {}_{}" , loan_idx . index () , region_name (loan . region) , location_name (loan . reserve_location) ,) ? ; } writeln ! (out , "") ? ; let mut points_per_region : FxIndexMap < RegionVid , FxIndexSet < PointIndex > > = FxIndexMap :: default () ; for constraint in & localized_outlives_constraints . outlives { points_per_region . entry (constraint . source) . or_default () . insert (constraint . from) ; points_per_region . entry (constraint . target) . or_default () . insert (constraint . to) ; } for (region , points) in points_per_region { writeln ! (out , "    subgraph \"{}\"" , region_name (region)) ? ; for point in points { writeln ! (out , "        {}" , node_name (region , point)) ? ; } writeln ! (out , "    end\n") ? ; } for constraint in & localized_outlives_constraints . outlives { writeln ! (out , "    {} --> {}" , node_name (constraint . source , constraint . from) , node_name (constraint . target , constraint . to) ,) ? ; } let edge_count = borrow_set . len () + localized_outlives_constraints . outlives . len () ; Ok (edge_count) }
    };
}

emit_mermaid_constraint_graph!();