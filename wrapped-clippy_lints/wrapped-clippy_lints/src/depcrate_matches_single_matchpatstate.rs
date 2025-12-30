// Generated macro for PatState (enum)
macro_rules! Depcrate_matches_single_matchPatState {
() => {
// Module: crate::matches::single_match
// Provides: {"PatState"}
// Dependencies: {}
# [doc = " State for tracking whether a match can become non-exhaustive by adding a variant to a contained"] # [doc = " enum."] # [doc = ""] # [doc = " This treats certain std enums as if they will never be extended."] enum PatState < 'a > { # [doc = " Either a wild match or an uninteresting type. Uninteresting types include:"] # [doc = " * builtin types (e.g. `i32` or `!`)"] # [doc = " * A struct/tuple/array containing only uninteresting types."] # [doc = " * A std enum containing only uninteresting types."] Wild , # [doc = " A std enum we know won't be extended. Tracks the states of each variant separately."] # [doc = ""] # [doc = " This is not used for `Option` since it uses the current pattern to track its state."] StdEnum (& 'a mut [Self]) , # [doc = " Either the initial state for a pattern or a non-std enum. There is currently no need to"] # [doc = " distinguish these cases."] # [doc = ""] # [doc = " For non-std enums there's no need to track the state of sub-patterns as the state of just"] # [doc = " this pattern on its own is enough for linting. Consider two cases:"] # [doc = " * This enum has no wild match. This case alone is enough to determine we can lint."] # [doc = " * This enum has a wild match and therefore all sub-patterns also have a wild match."] # [doc = ""] # [doc = " In both cases the sub patterns are not needed to determine whether to lint."] Other , }
};
}
