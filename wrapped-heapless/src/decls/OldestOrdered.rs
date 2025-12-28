macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! OldestOrdered {
    () => {
        deps!();
        # [doc = " Double ended iterator on the underlying buffer ordered from the oldest data"] # [doc = " to the newest."] pub struct OldestOrdered < 'a , T > { inner : core :: iter :: Chain < core :: slice :: Iter < 'a , T > , core :: slice :: Iter < 'a , T > > , }
    };
}

OldestOrdered!();