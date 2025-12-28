macro_rules! deps {
    () => {
        TryResult!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        impl < R > TryResult < R > { # [doc = " Returns `true` if the value was present in the map, and the lock for the shard was successfully obtained."] pub fn is_present (& self) -> bool { matches ! (self , TryResult :: Present (_)) } # [doc = " Returns `true` if the shard wasn't locked, and the value wasn't present in the map."] pub fn is_absent (& self) -> bool { matches ! (self , TryResult :: Absent) } # [doc = " Returns `true` if the shard was locked."] pub fn is_locked (& self) -> bool { matches ! (self , TryResult :: Locked) } # [doc = " If `self` is [Present](TryResult::Present), returns the reference to the value in the map."] # [doc = " Panics if `self` is not [Present](TryResult::Present)."] pub fn unwrap (self) -> R { match self { TryResult :: Present (r) => r , TryResult :: Locked => panic ! ("Called unwrap() on TryResult::Locked") , TryResult :: Absent => panic ! ("Called unwrap() on TryResult::Absent") , } } # [doc = " If `self` is [Present](TryResult::Present), returns the reference to the value in the map."] # [doc = " If `self` is not [Present](TryResult::Present), returns `None`."] pub fn try_unwrap (self) -> Option < R > { match self { TryResult :: Present (r) => Some (r) , _ => None , } } }
    };
}

impl_135!()