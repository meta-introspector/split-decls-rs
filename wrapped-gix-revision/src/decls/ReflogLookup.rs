macro_rules! ReflogLookup {
    () => {
        # [doc = " A lookup into the reflog of a reference."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] pub enum ReflogLookup { # [doc = " Lookup by entry, where `0` is the most recent entry, and `1` is the older one behind `0`."] Entry (usize) , # [doc = " Lookup the reflog at the given time and find the closest matching entry."] Date (gix_date :: Time) , }
    };
}

ReflogLookup!();