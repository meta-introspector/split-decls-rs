// Generated macro for IndexMapper (struct)
macro_rules! Depcrate_dfa_remapperIndexMapper {
() => {
// Module: crate::dfa::remapper
// Provides: {"IndexMapper"}
// Dependencies: {}
# [doc = " A simple type for mapping between state indices and state IDs."] # [doc = ""] # [doc = " The reason why this exists is because state IDs are \"premultiplied.\" That"] # [doc = " is, in order to get to the transitions for a particular state, one need"] # [doc = " only use the state ID as-is, instead of having to multiple it by transition"] # [doc = " table's stride."] # [doc = ""] # [doc = " The downside of this is that it's inconvenient to map between state IDs"] # [doc = " using a dense map, e.g., Vec<StateID>. That's because state IDs look like"] # [doc = " `0`, `0+stride`, `0+2*stride`, `0+3*stride`, etc., instead of `0`, `1`,"] # [doc = " `2`, `3`, etc."] # [doc = ""] # [doc = " Since our state IDs are premultiplied, we can convert back-and-forth"] # [doc = " between IDs and indices by simply unmultiplying the IDs and multiplying the"] # [doc = " indices."] # [derive (Debug)] struct IndexMapper { # [doc = " The power of 2 corresponding to the stride of the corresponding"] # [doc = " transition table. 'id >> stride2' de-multiplies an ID while 'index <<"] # [doc = " stride2' pre-multiplies an index to an ID."] stride2 : usize , }
};
}
