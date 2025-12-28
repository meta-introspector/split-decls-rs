macro_rules! deps {
    () => {
        DashMap!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl < 'a , K : 'a + Eq + Hash , V : 'a > DashMap < K , V , RandomState > { # [doc = " Creates a new DashMap with a capacity of 0."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use dashmap::DashMap;"] # [doc = ""] # [doc = " let reviews = DashMap::new();"] # [doc = " reviews.insert(\"Veloren\", \"What a fantastic game!\");"] # [doc = " ```"] pub fn new () -> Self { DashMap :: with_hasher (RandomState :: default ()) } # [doc = " Creates a new DashMap with a specified starting capacity."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use dashmap::DashMap;"] # [doc = ""] # [doc = " let mappings = DashMap::with_capacity(2);"] # [doc = " mappings.insert(2, 4);"] # [doc = " mappings.insert(8, 16);"] # [doc = " ```"] pub fn with_capacity (capacity : usize) -> Self { DashMap :: with_capacity_and_hasher (capacity , RandomState :: default ()) } # [doc = " Creates a new DashMap with a specified shard amount"] # [doc = ""] # [doc = " shard_amount should greater than 0 and be a power of two."] # [doc = " If a shard_amount which is not a power of two is provided, the function will panic."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use dashmap::DashMap;"] # [doc = ""] # [doc = " let mappings = DashMap::with_shard_amount(32);"] # [doc = " mappings.insert(2, 4);"] # [doc = " mappings.insert(8, 16);"] # [doc = " ```"] pub fn with_shard_amount (shard_amount : usize) -> Self { Self :: with_capacity_and_hasher_and_shard_amount (0 , RandomState :: default () , shard_amount) } # [doc = " Creates a new DashMap with a specified capacity and shard amount."] # [doc = ""] # [doc = " shard_amount should greater than 0 and be a power of two."] # [doc = " If a shard_amount which is not a power of two is provided, the function will panic."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use dashmap::DashMap;"] # [doc = ""] # [doc = " let mappings = DashMap::with_capacity_and_shard_amount(32, 32);"] # [doc = " mappings.insert(2, 4);"] # [doc = " mappings.insert(8, 16);"] # [doc = " ```"] pub fn with_capacity_and_shard_amount (capacity : usize , shard_amount : usize) -> Self { Self :: with_capacity_and_hasher_and_shard_amount (capacity , RandomState :: default () , shard_amount ,) } }
    };
}

impl_157!();