// Generated macro for duplicate_arguments (function)
macro_rules! Depcrate_errorduplicate_arguments {
() => {
// Module: crate::error
// Provides: {"duplicate_arguments"}
// Dependencies: {}
fn duplicate_arguments < 'a , I : MaybePat + Spanned + 'a > (args : impl Iterator < Item = & 'a I > + 'a ,) -> Errors < 'a > { let mut used = HashMap :: new () ; Box :: new (args . filter_map (| it | it . maybe_pat () . map (| pat | (it , pat))) . filter_map (move | (it , pat) | { let is_duplicate = used . contains_key (& pat) ; used . insert (pat , it) ; match is_duplicate { true => Some ((it , pat)) , false => None , } }) . map (| (duplicate , pat) | { syn :: Error :: new (duplicate . span () , format ! ("Duplicate argument: '{}' is already defined." , pat . render_type ()) ,) }) ,) }
};
}
