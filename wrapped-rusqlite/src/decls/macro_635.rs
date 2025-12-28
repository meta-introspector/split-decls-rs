macro_rules! macro_635 {
    () => {
        bitflags :: bitflags ! { # [derive (Clone , Copy)] # [repr (C)] struct QueryPlanFlags : c_int { const START = 1 ; const STOP = 2 ; const STEP = 4 ; const DESC = 8 ; const ASC = 16 ; const BOTH = QueryPlanFlags :: START . bits () | QueryPlanFlags :: STOP . bits () ; } }
    };
}

macro_635!();