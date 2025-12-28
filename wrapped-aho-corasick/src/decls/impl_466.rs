macro_rules! deps {
    () => {
        StateID!();
        Special!();
    };
}

macro_rules! impl_466 {
    () => {
        deps!();
        impl Special { # [doc = " Create a new set of \"special\" state IDs with all IDs initialized to"] # [doc = " zero. The general idea here is that they will be updated and set to"] # [doc = " correct values later."] pub (crate) fn zero () -> Special { Special { max_special_id : StateID :: ZERO , max_match_id : StateID :: ZERO , start_unanchored_id : StateID :: ZERO , start_anchored_id : StateID :: ZERO , } } }
    };
}

impl_466!();