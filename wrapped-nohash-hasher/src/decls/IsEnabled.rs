macro_rules! deps {
    () => {
        NoHashHasher!();
    };
}

macro_rules! IsEnabled {
    () => {
        deps!();
        # [doc = " Types which are safe to use with `NoHashHasher`."] # [doc = ""] # [doc = " This marker trait is an option for types to enable themselves for use"] # [doc = " with `NoHashHasher`. In order to be safe, the `Hash` impl needs to"] # [doc = " satisfy the following constraint:"] # [doc = ""] # [doc = " > **One of the `Hasher::write_{u8,u16,u32,u64,usize,i8,i16,i32,i64,isize}`"] # [doc = " methods is invoked exactly once.**"] # [doc = ""] # [doc = " The best way to ensure this is to write a custom `Hash` impl even when"] # [doc = " deriving `Hash` for a simple newtype of a single type which itself"] # [doc = " implements `IsEnabled` may work as well."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " #[derive(PartialEq, Eq)]"] # [doc = " struct SomeType(u32);"] # [doc = ""] # [doc = " impl std::hash::Hash for SomeType {"] # [doc = "     fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {"] # [doc = "         hasher.write_u32(self.0)"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " impl nohash_hasher::IsEnabled for SomeType {}"] # [doc = ""] # [doc = " let mut m = nohash_hasher::IntMap::default();"] # [doc = ""] # [doc = " m.insert(SomeType(1), 't');"] # [doc = " m.insert(SomeType(0), 'f');"] # [doc = ""] # [doc = " assert_eq!(Some(&'t'), m.get(&SomeType(1)));"] # [doc = " assert_eq!(Some(&'f'), m.get(&SomeType(0)));"] # [doc = " ```"] pub trait IsEnabled { }
    };
}

IsEnabled!()