// Generated macro for scan_reference (function)
macro_rules! Depcrate_parsescan_reference {
() => {
// Module: crate::parse
// Provides: {"scan_reference"}
// Dependencies: {}
fn scan_reference < 'b > (tree : & Tree < Item > , text : & 'b str , cur : Option < TreeIndex > , options : Options ,) -> RefScan < 'b > { let cur_ix = match cur { None => return RefScan :: Failed , Some (cur_ix) => cur_ix , } ; let start = tree [cur_ix] . item . start ; let tail = & text . as_bytes () [start ..] ; if tail . starts_with (b"[]") { let closing_node = tree [cur_ix] . next . unwrap () ; RefScan :: Collapsed (tree [closing_node] . next) } else { let label = scan_link_label (tree , & text [start ..] , options) ; match label { Some ((ix , ReferenceLabel :: Link (label))) => RefScan :: LinkLabel (label , start + ix) , Some ((_ix , ReferenceLabel :: Footnote (_label))) => RefScan :: UnexpectedFootnote , None => RefScan :: Failed , } } }
};
}
