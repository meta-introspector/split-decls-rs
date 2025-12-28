macro_rules! deps {
    () => {
        ModPath!();
        Display!();
        PathKind!();
        ExpandDatabase!();
    };
}

macro_rules! display_fmt_path {
    () => {
        deps!();
        fn display_fmt_path (db : & dyn ExpandDatabase , path : & ModPath , f : & mut fmt :: Formatter < '_ > , edition : Option < Edition > ,) -> fmt :: Result { let mut first_segment = true ; let mut add_segment = | s | -> fmt :: Result { if ! first_segment { f . write_str ("::") ? ; } first_segment = false ; f . write_str (s) ? ; Ok (()) } ; match path . kind { PathKind :: Plain => { } PathKind :: SELF => add_segment ("self") ? , PathKind :: Super (n) => { for _ in 0 .. n { add_segment ("super") ? ; } } PathKind :: Crate => add_segment ("crate") ? , PathKind :: Abs => add_segment ("") ? , PathKind :: DollarCrate (_) => add_segment ("$crate") ? , } for segment in & path . segments { if ! first_segment { f . write_str ("::") ? ; } first_segment = false ; match edition { Some (edition) => segment . display (db , edition) . fmt (f) ? , None => fmt :: Display :: fmt (segment . as_str () , f) ? , } ; } Ok (()) }
    };
}

display_fmt_path!();