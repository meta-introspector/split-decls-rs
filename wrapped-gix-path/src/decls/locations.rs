macro_rules! locations {
    () => {
        # [cfg (not (windows))] mod locations { # [test] fn alternative_locations () { assert ! (super :: super :: ALTERNATIVE_LOCATIONS . is_empty ()) ; } }
    };
}

locations!();