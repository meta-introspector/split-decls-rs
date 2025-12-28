macro_rules! deps {
    () => {
        FxHasher!();
    };
}

macro_rules! FxSeededState {
    () => {
        deps!();
        # [doc = " [`FxSeededState`] is an alternative state for `HashMap` types, allowing to use [`FxHasher`] with a set seed."] # [doc = ""] # [doc = " ```"] # [doc = " # use std::collections::HashMap;"] # [doc = " use rustc_hash::FxSeededState;"] # [doc = ""] # [doc = " let mut map = HashMap::with_hasher(FxSeededState::with_seed(12));"] # [doc = " map.insert(15, 610);"] # [doc = " assert_eq!(map[&15], 610);"] # [doc = " ```"] # [derive (Clone)] pub struct FxSeededState { seed : usize , }
    };
}

FxSeededState!();