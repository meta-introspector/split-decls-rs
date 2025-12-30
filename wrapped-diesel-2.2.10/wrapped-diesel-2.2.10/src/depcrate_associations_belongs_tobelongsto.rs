// Generated macro for BelongsTo (trait)
macro_rules! Depcrate_associations_belongs_toBelongsTo {
() => {
// Module: crate::associations::belongs_to
// Provides: {"BelongsTo"}
// Dependencies: {}
# [doc = " Indicates that a type belongs to `Parent`"] # [doc = ""] # [doc = " Specifically, this means that this struct has fields"] # [doc = " which correspond to the primary key of `Parent`."] # [doc = " This implies that a foreign key relationship exists on the tables."] # [doc = ""] # [doc = " This trait is not capable of supporting composite foreign keys"] pub trait BelongsTo < Parent > { # [doc = " The foreign key of this struct"] type ForeignKey : Hash + :: std :: cmp :: Eq ; # [doc = " The database column representing the foreign key"] # [doc = " of the table this struct represents"] type ForeignKeyColumn : Column ; # [doc = " Returns the foreign key for `self`"] fn foreign_key (& self) -> Option < & Self :: ForeignKey > ; # [doc = " Returns the foreign key column of this struct's table"] fn foreign_key_column () -> Self :: ForeignKeyColumn ; }
};
}
