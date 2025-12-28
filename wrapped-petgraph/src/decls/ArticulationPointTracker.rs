macro_rules! ArticulationPointTracker {
    () => {
        # [doc = " Internal auxiliary helper struct for global variables."] struct ArticulationPointTracker { visited : FixedBitSet , low : Vec < usize > , disc : Vec < usize > , parent : Vec < usize > , time : usize , articulation_points : HashSet < usize > , }
    };
}

ArticulationPointTracker!()