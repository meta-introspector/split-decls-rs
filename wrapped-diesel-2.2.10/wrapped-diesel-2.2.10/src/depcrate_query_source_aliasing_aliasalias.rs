// Generated macro for Alias (struct)
macro_rules! Depcrate_query_source_aliasing_aliasAlias {
() => {
// Module: crate::query_source::aliasing::alias
// Provides: {"Alias"}
// Dependencies: {}
# [derive (Debug , Clone , Copy , Default)] # [doc = " Represents an alias within diesel's query builder"] # [doc = ""] # [doc = " See [`alias!`](crate::alias) for more details."] # [diesel_derives :: __diesel_public_if (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes" , public_fields (source))] pub struct Alias < S > { # [doc = " The inner alias definition"] pub (crate) source : S , }
};
}
