macro_rules! deps {
    () => {
        Str!();
    };
}

macro_rules! Id {
    () => {
        deps!();
        # [doc = " [`Arg`][crate::Arg] or [`ArgGroup`][crate::ArgGroup] identifier"] # [doc = ""] # [doc = " This is used for accessing the value in [`ArgMatches`][crate::ArgMatches] or defining"] # [doc = " relationships between `Arg`s and `ArgGroup`s with functions like"] # [doc = " [`Arg::conflicts_with`][crate::Arg::conflicts_with]."] # [derive (Default , Clone , Eq , PartialEq , PartialOrd , Ord , Hash)] pub struct Id (Str) ;
    };
}

Id!()