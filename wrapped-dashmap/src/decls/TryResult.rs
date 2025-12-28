macro_rules! TryResult {
    () => {
        # [doc = " Represents the result of a non-blocking read from a [DashMap](crate::DashMap)."] # [derive (Debug)] pub enum TryResult < R > { # [doc = " The value was present in the map, and the lock for the shard was successfully obtained."] Present (R) , # [doc = " The shard wasn't locked, and the value wasn't present in the map."] Absent , # [doc = " The shard was locked."] Locked , }
    };
}

TryResult!();