macro_rules! deps {
    () => {
        Time!();
    };
}

macro_rules! IndexTime {
    () => {
        deps!();
        # [doc = " Time structure used in a git index entry."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub struct IndexTime { raw : raw :: git_index_time , }
    };
}

IndexTime!();