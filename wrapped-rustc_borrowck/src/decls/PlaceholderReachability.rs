macro_rules! PlaceholderReachability {
    () => {
        # [derive (Copy , Debug , Clone , PartialEq , Eq)] enum PlaceholderReachability { # [doc = " This SCC reaches no placeholders."] NoPlaceholders , # [doc = " This SCC reaches at least one placeholder."] Placeholders { # [doc = " The largest-universed placeholder we can reach"] max_universe : (UniverseIndex , RegionVid) , # [doc = " The placeholder with the smallest ID"] min_placeholder : RegionVid , # [doc = " The placeholder with the largest ID"] max_placeholder : RegionVid , } , }
    };
}

PlaceholderReachability!();