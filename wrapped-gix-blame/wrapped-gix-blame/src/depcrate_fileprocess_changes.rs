// Generated macro for process_changes (function)
macro_rules! Depcrate_fileprocess_changes {
() => {
// Module: crate::file
// Provides: {"process_changes"}
// Dependencies: {}
# [doc = " Consume `hunks_to_blame` and `changes` to pair up matches ranges (also overlapping) with each other."] # [doc = " Once a match is found, it's pushed onto `out`."] # [doc = ""] # [doc = " `process_changes` assumes that ranges coming from the same *Source File* can and do"] # [doc = " occasionally overlap. If it were a desirable property of the blame algorithm as a whole to"] # [doc = " never have two different lines from a *Blamed File* mapped to the same line in a *Source File*,"] # [doc = " this property would need to be enforced at a higher level than `process_changes`."] # [doc = " Then the nested loops could potentially be flattened into one."] fn process_changes (hunks_to_blame : Vec < UnblamedHunk > , changes : Vec < Change > , suspect : ObjectId , parent : ObjectId ,) -> Vec < UnblamedHunk > { let mut new_hunks_to_blame = Vec :: new () ; for mut hunk in hunks_to_blame . into_iter () . map (Some) { let mut offset_in_destination = Offset :: Added (0) ; let mut changes_iter = changes . iter () . cloned () ; let mut change = changes_iter . next () ; loop { (hunk , change) = process_change (& mut new_hunks_to_blame , & mut offset_in_destination , suspect , parent , hunk , change ,) ; change = change . or_else (| | changes_iter . next ()) ; if hunk . is_none () { break ; } } } new_hunks_to_blame }
};
}
