macro_rules! Direction {
    () => {
        # [doc = " An enumeration of the possible directions for a remote."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum Direction { # [doc = " Data will be fetched (read) from this remote."] Fetch , # [doc = " Data will be pushed (written) to this remote."] Push , }
    };
}

Direction!()