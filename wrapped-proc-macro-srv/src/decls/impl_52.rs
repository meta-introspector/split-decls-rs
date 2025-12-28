macro_rules! deps {
    () => {
        EnvSnapshot!();
        EnvChange!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < 'snap > EnvChange < 'snap > { fn apply (snap : & 'snap EnvSnapshot , new_vars : & 'snap [(String , String)] , current_dir : Option < & Path > ,) -> EnvChange < 'snap > { let guard = ENV_LOCK . lock () . unwrap_or_else (std :: sync :: PoisonError :: into_inner) ; let prev_working_dir = match current_dir { Some (dir) => { let prev_working_dir = std :: env :: current_dir () . ok () ; if let Err (err) = std :: env :: set_current_dir (dir) { eprintln ! ("Failed to set the current working dir to {}. Error: {err:?}" , dir . display ()) } prev_working_dir } None => None , } ; EnvChange { snap , changed_vars : new_vars . iter () . map (| (k , v) | { unsafe { env :: set_var (k , v) } ; & * * k }) . collect () , prev_working_dir , _guard : guard , } } fn rollback (self) { } }
    };
}

impl_52!();