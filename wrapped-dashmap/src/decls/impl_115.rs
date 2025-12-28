macro_rules! deps {
    () => {
        DashSet!();
        DashMap!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl < 'a , K : 'a + Eq + Hash > DashSet < K , RandomState > { # [doc = " Creates a new DashSet with a capacity of 0."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use dashmap::DashSet;"] # [doc = ""] # [doc = " let games = DashSet::new();"] # [doc = " games.insert(\"Veloren\");"] # [doc = " ```"] pub fn new () -> Self { Self :: with_hasher (RandomState :: default ()) } # [doc = " Creates a new DashMap with a specified starting capacity."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use dashmap::DashSet;"] # [doc = ""] # [doc = " let numbers = DashSet::with_capacity(2);"] # [doc = " numbers.insert(2);"] # [doc = " numbers.insert(8);"] # [doc = " ```"] pub fn with_capacity (capacity : usize) -> Self { Self :: with_capacity_and_hasher (capacity , RandomState :: default ()) } }
    };
}

impl_115!();