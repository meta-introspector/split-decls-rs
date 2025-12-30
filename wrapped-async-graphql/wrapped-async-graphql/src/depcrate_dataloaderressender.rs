// Generated macro for ResSender (struct)
macro_rules! Depcrate_dataloaderResSender {
() => {
// Module: crate::dataloader
// Provides: {"ResSender"}
// Dependencies: {}
# [allow (clippy :: type_complexity)] struct ResSender < K : Send + Sync + Hash + Eq + Clone + 'static , T : Loader < K > > { use_cache_values : HashMap < K , T :: Value > , tx : oneshot :: Sender < Result < HashMap < K , T :: Value > , T :: Error > > , }
};
}
