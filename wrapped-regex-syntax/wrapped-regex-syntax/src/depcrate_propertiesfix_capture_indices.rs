// Generated macro for fix_capture_indices (function)
macro_rules! Depcrate_propertiesfix_capture_indices {
() => {
// Module: crate::properties
// Provides: {"fix_capture_indices"}
// Dependencies: {}
fn fix_capture_indices (e : Expr) -> Expr { fn bx (e : Expr) -> Box < Expr > { Box :: new (e) } fn fix (e : Expr , capi : & mut usize , names : & mut Vec < String >) -> Expr { use Expr :: * ; match e { Group { e , i : Some (_) , mut name } => { * capi += 1 ; let i = * capi ; let mut dupe_name = false ; if let Some (ref n1) = name { if names . iter () . any (| n2 | n1 == n2) { dupe_name = true ; } else { names . push (n1 . clone ()) ; } } if dupe_name { name = None ; } Group { e : bx (fix (* e , capi , names)) , i : Some (i) , name : name } } Group { e , i , name } => { Group { e : bx (fix (* e , capi , names)) , i : i , name : name } } Repeat { e , r , greedy } => { Repeat { e : bx (fix (* e , capi , names)) , r : r , greedy : greedy } } Concat (es) => Concat (es . into_iter () . map (| e | fix (e , capi , names)) . collect ()) , Alternate (es) => Alternate (es . into_iter () . map (| e | fix (e , capi , names)) . collect ()) , e => e , } } fix (e , & mut 0 , & mut vec ! []) }
};
}
