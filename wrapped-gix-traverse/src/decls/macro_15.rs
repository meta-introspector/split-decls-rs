macro_rules! deps {
    () => {
        Parents!();
    };
}

macro_rules! macro_15 {
    () => {
        deps!();
        bitflags ! { # [doc = " Set of flags to describe the state of a particular commit while iterating."] # [repr (transparent)] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub (super) struct WalkFlags : u8 { # [doc = " Commit has been seen"] const Seen = 0b000001 ; # [doc = " Commit has been processed by the Explore walk"] const Explored = 0b000010 ; # [doc = " Commit has been processed by the Indegree walk"] const InDegree = 0b000100 ; # [doc = " Commit is deemed uninteresting for whatever reason"] const Uninteresting = 0b001000 ; # [doc = " Commit marks the end of a walk, like `foo` in `git rev-list foo..bar`"] const Bottom = 0b010000 ; # [doc = " Parents have been processed"] const Added = 0b100000 ; } }
    };
}

macro_15!();