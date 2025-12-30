// Generated macro for Options (struct)
macro_rules! Depcrate_describeOptions {
() => {
// Module: crate::describe
// Provides: {"Options"}
// Dependencies: {}
# [doc = " The options required to call [`describe()`][function::describe()]."] # [derive (Clone , Debug)] pub struct Options < 'name > { # [doc = " The candidate names from which to determine the `name` to use for the describe string,"] # [doc = " as a mapping from a commit id and the name associated with it."] pub name_by_oid : HashMap < gix_hash :: ObjectId , Cow < 'name , BStr > > , # [doc = " The amount of names we will keep track of. Defaults to the maximum of 32."] # [doc = ""] # [doc = " If the number is exceeded, it will be capped at 32 and defaults to 10."] pub max_candidates : usize , # [doc = " If no candidate for naming, always show the abbreviated hash. Default: false."] pub fallback_to_oid : bool , # [doc = " Only follow the first parent during graph traversal. Default: false."] # [doc = ""] # [doc = " This may speed up the traversal at the cost of accuracy."] pub first_parent : bool , }
};
}
