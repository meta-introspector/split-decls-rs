macro_rules! deps {
    () => {
        Output!();
        VersionError!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl :: std :: error :: Error for VersionError { fn description (& self) -> & str { match self { VersionError :: Exec (_) => "Execution Error" , VersionError :: Error (_) => "Other Error" , VersionError :: OutputError => "Output Error" , VersionError :: ParseError (_) => "Parse Error" , } } fn cause (& self) -> Option < & dyn :: std :: error :: Error > { match self { VersionError :: Exec (err) => Some (err) , _ => None , } } }
    };
}

impl_58!()