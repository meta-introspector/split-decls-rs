// Generated macro for impl_428 (impl)
macro_rules! Depcrate_concurrency_data_race_handlerimpl_428 {
() => {
// Module: crate::concurrency::data_race_handler
// Provides: {"impl_428"}
// Dependencies: {}
impl AllocDataRaceHandler { pub fn as_vclocks_ref (& self) -> Option < & data_race :: AllocState > { if let Self :: Vclocks (data_race , _weak_memory) = self { Some (data_race) } else { None } } pub fn as_vclocks_mut (& mut self) -> Option < & mut data_race :: AllocState > { if let Self :: Vclocks (data_race , _weak_memory) = self { Some (data_race) } else { None } } pub fn as_weak_memory_ref (& self) -> Option < & weak_memory :: AllocState > { if let Self :: Vclocks (_data_race , weak_memory) = self { weak_memory . as_ref () } else { None } } pub fn as_weak_memory_mut (& mut self) -> Option < & mut weak_memory :: AllocState > { if let Self :: Vclocks (_data_race , weak_memory) = self { weak_memory . as_mut () } else { None } } }
};
}
