// Generated macro for apply_change (function)
macro_rules! Depcrate_tree_utilsapply_change {
() => {
// Module: crate::tree::utils
// Provides: {"apply_change"}
// Dependencies: {}
# [doc = " Unconditionally apply `change` to `editor`."] pub fn apply_change (editor : & mut tree :: Editor < '_ > , change : & Change , alternative_location : Option < & BString > ,) -> Result < () , tree :: editor :: Error > { use to_components_bstring_ref as to_components ; if change . entry_mode () . is_tree () { return Ok (()) ; } let (location , mode , id) = match change { Change :: Addition { location , entry_mode , id , .. } | Change :: Modification { location , entry_mode , id , .. } => (location , entry_mode , id) , Change :: Deletion { location , .. } => { editor . remove (to_components (alternative_location . unwrap_or (location))) ? ; return Ok (()) ; } Change :: Rewrite { source_location , entry_mode , id , location , copy , .. } => { if ! * copy { editor . remove (to_components (source_location)) ? ; } (location , entry_mode , id) } } ; editor . upsert (to_components (alternative_location . unwrap_or (location)) , mode . kind () , * id ,) ? ; Ok (()) }
};
}
