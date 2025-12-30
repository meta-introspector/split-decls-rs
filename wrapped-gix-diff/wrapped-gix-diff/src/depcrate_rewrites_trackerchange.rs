// Generated macro for Change (trait)
macro_rules! Depcrate_rewrites_trackerChange {
() => {
// Module: crate::rewrites::tracker
// Provides: {"Change"}
// Dependencies: {}
# [doc = " A trait providing all functionality to abstract over the concept of a change, as seen by the [`Tracker`]."] pub trait Change : Clone { # [doc = " Return the hash of the object behind this change for identification."] # [doc = ""] # [doc = " Note that this is the id of the object as stored in `git`, i.e. it must have gone through workspace"] # [doc = " conversions. What matters is that the IDs are comparable."] fn id (& self) -> & gix_hash :: oid ; # [doc = " Return the relation that this change may have with other changes."] # [doc = ""] # [doc = " It allows to associate a directory with its children that are added or removed at the same moment."] # [doc = " Note that this is ignored for modifications."] # [doc = ""] # [doc = " If rename-tracking should always be on leaf-level, this should be set to `None` consistently."] # [doc = " Note that trees will never be looked up by their `id` as their children are assumed to be passed in"] # [doc = " with the respective relationship."] # [doc = ""] # [doc = " Also note that the tracker only sees what's given to it, it will not lookup trees or match paths itself."] fn relation (& self) -> Option < Relation > ; # [doc = " Return the kind of this change."] fn kind (& self) -> ChangeKind ; # [doc = " Return more information about the kind of entry affected by this change."] fn entry_mode (& self) -> EntryMode ; # [doc = " Return the id of the change along with its mode."] fn id_and_entry_mode (& self) -> (& gix_hash :: oid , EntryMode) ; }
};
}
