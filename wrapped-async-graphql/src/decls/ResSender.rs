macro_rules! deps {
    () => {
        Loader!();
        Error!();
        Result!();
    };
}

macro_rules! ResSender {
    () => {
        deps!();
        # [allow (clippy :: type_complexity)] struct ResSender < K : Send + Sync + Hash + Eq + Clone + 'static , T : Loader < K > > { use_cache_values : HashMap < K , T :: Value > , tx : oneshot :: Sender < Result < HashMap < K , T :: Value > , T :: Error > > , }
    };
}

ResSender!()