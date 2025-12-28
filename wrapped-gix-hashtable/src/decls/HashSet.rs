macro_rules! deps {
    () => {
        HashMap!();
    };
}

macro_rules! HashSet {
    () => {
        deps!();
        # [doc = " A `HashSet` for usage with keys that are already robust hashes (like an `ObjectId`)."] # [doc = " The first `8` bytes of the hash are used as the `HashMap` hash"] pub type HashSet < T = ObjectId > = hashbrown :: HashSet < T , hash :: Builder > ;
    };
}

HashSet!()