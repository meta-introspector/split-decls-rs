macro_rules! deps {
    () => {
        DashMap!();
    };
}

macro_rules! DashSet {
    () => {
        deps!();
        # [doc = " DashSet is a thin wrapper around [`DashMap`] using `()` as the value type. It uses"] # [doc = " methods and types which are more convenient to work with on a set."] # [doc = ""] # [doc = " [`DashMap`]: struct.DashMap.html"] pub struct DashSet < K , S = RandomState > { pub (crate) inner : DashMap < K , () , S > , }
    };
}

DashSet!()