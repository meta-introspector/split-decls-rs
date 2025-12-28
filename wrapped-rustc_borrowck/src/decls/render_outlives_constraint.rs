macro_rules! deps {
    () => {
        Locations!();
        OutlivesConstraint!();
    };
}

macro_rules! render_outlives_constraint {
    () => {
        deps!();
        fn render_outlives_constraint (constraint : & OutlivesConstraint < '_ >) -> String { if let ConstraintCategory :: OutlivesUnnameablePlaceholder (unnameable) = constraint . category { format ! ("{unnameable:?} unnameable") } else { match constraint . locations { Locations :: All (_) => "All(...)" . to_string () , Locations :: Single (loc) => format ! ("{loc:?}") , } } }
    };
}

render_outlives_constraint!()