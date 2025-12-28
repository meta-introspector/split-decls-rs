macro_rules! FxRandomState {
    () => {
        # [doc = " `FxRandomState` is an alternative state for `HashMap` types."] # [doc = ""] # [doc = " A particular instance `FxRandomState` will create the same instances of"] # [doc = " [`Hasher`], but the hashers created by two different `FxRandomState`"] # [doc = " instances are unlikely to produce the same result for the same values."] # [derive (Clone)] pub struct FxRandomState { seed : usize , }
    };
}

FxRandomState!()