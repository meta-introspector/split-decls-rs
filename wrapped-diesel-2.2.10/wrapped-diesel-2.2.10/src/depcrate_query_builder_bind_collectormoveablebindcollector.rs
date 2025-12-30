// Generated macro for MoveableBindCollector (trait)
macro_rules! Depcrate_query_builder_bind_collectorMoveableBindCollector {
() => {
// Module: crate::query_builder::bind_collector
// Provides: {"MoveableBindCollector"}
// Dependencies: {}
# [doc = " A movable version of the bind collector which allows it to be extracted, moved and refilled."] # [doc = ""] # [doc = " This is mostly useful in async context where bind data needs to be moved across threads."] # [diesel_derives :: __diesel_public_if (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")] pub trait MoveableBindCollector < DB : TypeMetadata > { # [doc = " The movable bind data of this bind collector"] type BindData : Send + 'static ; # [doc = " Builds a movable version of the bind collector"] fn moveable (& self) -> Self :: BindData ; # [doc = " Refill the bind collector with its bind data"] fn append_bind_data (& mut self , from : & Self :: BindData) ; }
};
}
