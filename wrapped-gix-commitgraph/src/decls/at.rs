macro_rules! deps {
    () => {
        Graph!();
        Error!();
    };
}

macro_rules! at {
    () => {
        deps!();
        # [doc = " Instantiate a commit graph from an `.git/objects/info` directory, or one of the various commit-graph files."] pub fn at (path : impl AsRef < Path >) -> Result < Graph , init :: Error > { Graph :: at (path . as_ref ()) }
    };
}

at!();