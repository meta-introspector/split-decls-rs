// Generated macro for merge_repr (function)
macro_rules! Depcrate_attrmerge_repr {
() => {
// Module: crate::attr
// Provides: {"merge_repr"}
// Dependencies: {}
fn merge_repr (this : & mut ReprOptions , other : ReprOptions) { let ReprOptions { int , align , pack , flags , field_shuffle_seed : _ } = this ; flags . insert (other . flags) ; * align = (* align) . max (other . align) ; * pack = match (* pack , other . pack) { (Some (pack) , None) | (None , Some (pack)) => Some (pack) , _ => (* pack) . min (other . pack) , } ; if other . int . is_some () { * int = other . int ; } }
};
}
