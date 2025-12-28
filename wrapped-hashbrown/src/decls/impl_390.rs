macro_rules! deps {
    () => {
        HashSet!();
        HashMap!();
        DefaultHashBuilder!();
    };
}

macro_rules! impl_390 {
    () => {
        deps!();
        # [cfg (feature = "default-hasher")] impl < T > HashSet < T , DefaultHashBuilder > { # [doc = " Creates an empty `HashSet`."] # [doc = ""] # [doc = " The hash set is initially created with a capacity of 0, so it will not allocate until it"] # [doc = " is first inserted into."] # [doc = ""] # [doc = " # HashDoS resistance"] # [doc = ""] # [doc = " The `hash_builder` normally use a fixed key by default and that does"] # [doc = " not allow the `HashSet` to be protected against attacks such as [`HashDoS`]."] # [doc = " Users who require HashDoS resistance should explicitly use"] # [doc = " [`std::collections::hash_map::RandomState`]"] # [doc = " as the hasher when creating a [`HashSet`], for example with"] # [doc = " [`with_hasher`](HashSet::with_hasher) method."] # [doc = ""] # [doc = " [`HashDoS`]: https://en.wikipedia.org/wiki/Collision_attack"] # [doc = " [`std::collections::hash_map::RandomState`]: https://doc.rust-lang.org/std/collections/hash_map/struct.RandomState.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::HashSet;"] # [doc = " let set: HashSet<i32> = HashSet::new();"] # [doc = " ```"] # [cfg_attr (feature = "inline-more" , inline)] pub fn new () -> Self { Self { map : HashMap :: new () , } } # [doc = " Creates an empty `HashSet` with the specified capacity."] # [doc = ""] # [doc = " The hash set will be able to hold at least `capacity` elements without"] # [doc = " reallocating. If `capacity` is 0, the hash set will not allocate."] # [doc = ""] # [doc = " # HashDoS resistance"] # [doc = ""] # [doc = " The `hash_builder` normally use a fixed key by default and that does"] # [doc = " not allow the `HashSet` to be protected against attacks such as [`HashDoS`]."] # [doc = " Users who require HashDoS resistance should explicitly use"] # [doc = " [`std::collections::hash_map::RandomState`]"] # [doc = " as the hasher when creating a [`HashSet`], for example with"] # [doc = " [`with_capacity_and_hasher`](HashSet::with_capacity_and_hasher) method."] # [doc = ""] # [doc = " [`HashDoS`]: https://en.wikipedia.org/wiki/Collision_attack"] # [doc = " [`std::collections::hash_map::RandomState`]: https://doc.rust-lang.org/std/collections/hash_map/struct.RandomState.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::HashSet;"] # [doc = " let set: HashSet<i32> = HashSet::with_capacity(10);"] # [doc = " assert!(set.capacity() >= 10);"] # [doc = " ```"] # [cfg_attr (feature = "inline-more" , inline)] pub fn with_capacity (capacity : usize) -> Self { Self { map : HashMap :: with_capacity (capacity) , } } }
    };
}

impl_390!()