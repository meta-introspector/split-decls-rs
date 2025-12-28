macro_rules! deps {
    () => {
        Replacements!();
        Error!();
    };
}

macro_rules! handle_errors_and_replacements {
    () => {
        deps!();
        fn handle_errors_and_replacements (destination : & mut Vec < Error > , objs : & mut HashSet < ObjectId > , errors : Vec < (ObjectId , Error) > , replacements : & mut Replacements ,) -> Option < () > { if errors . len () == objs . len () { destination . extend (errors . into_iter () . map (| (_ , err) | err)) ; None } else { for (obj , err) in errors { objs . remove (& obj) ; destination . push (err) ; } for (find , replace) in replacements { objs . remove (find) ; objs . insert (* replace) ; } Some (()) } }
    };
}

handle_errors_and_replacements!()