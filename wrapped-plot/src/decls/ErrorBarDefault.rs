macro_rules! deps {
    () => {
        Default!();
    };
}

macro_rules! ErrorBarDefault {
    () => {
        deps!();
        # [doc = " Error bar variant of Default"] trait ErrorBarDefault < S > { # [doc = " Creates `errorbar::Properties` with default configuration"] fn default (s : S) -> Self ; }
    };
}

ErrorBarDefault!()