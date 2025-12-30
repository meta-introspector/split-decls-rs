// Generated macro for AliasSource (trait)
macro_rules! Depcrate_query_source_aliasingAliasSource {
() => {
// Module: crate::query_source::aliasing
// Provides: {"AliasSource"}
// Dependencies: {}
# [doc = " Types created by the `alias!` macro that serve to distinguish between aliases implement"] # [doc = " this trait."] # [doc = ""] # [doc = " In order to be able to implement within diesel a lot of traits on what will represent the alias,"] # [doc = " we cannot use directly that new type within the query builder. Instead, we will use `Alias<S>`,"] # [doc = " where `S: AliasSource`."] # [doc = ""] # [doc = " This trait should never be implemented by an end-user directly."] pub trait AliasSource { # [doc = " The name of this alias in the query"] const NAME : & 'static str ; # [doc = " The table the alias maps to"] type Target ; # [doc = " Obtain the table from the source"] # [doc = ""] # [doc = " (used by Diesel to implement some traits)"] fn target (& self) -> & Self :: Target ; }
};
}
