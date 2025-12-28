macro_rules! deps {
    () => {
        HashMap!();
        HashSet!();
    };
}

macro_rules! DefaultHashBuilder {
    () => {
        deps!();
        # [doc = " Default hash builder for the `S` type parameter of"] # [doc = " [`HashMap`](crate::HashMap) and [`HashSet`](crate::HashSet)."] # [doc = ""] # [doc = " This only implements `BuildHasher` when the \"default-hasher\" crate feature"] # [doc = " is enabled; otherwise it just serves as a placeholder, and a custom `S` type"] # [doc = " must be used to have a fully functional `HashMap` or `HashSet`."] # [derive (Clone , Debug , Default)] pub struct DefaultHashBuilder { # [cfg (feature = "default-hasher")] inner : RandomState , }
    };
}

DefaultHashBuilder!();