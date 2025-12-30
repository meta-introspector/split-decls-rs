// Generated macro for impl_71 (impl)
macro_rules! Depcrate_borrow_setimpl_71 {
() => {
// Module: crate::borrow_set
// Provides: {"impl_71"}
// Dependencies: {}
impl < 'tcx > BorrowSet < 'tcx > { pub fn build (tcx : TyCtxt < 'tcx > , body : & Body < 'tcx > , locals_are_invalidated_at_exit : bool , move_data : & MoveData < 'tcx > ,) -> Self { let mut visitor = GatherBorrows { tcx , body , location_map : Default :: default () , activation_map : Default :: default () , local_map : Default :: default () , pending_activations : Default :: default () , locals_state_at_exit : LocalsStateAtExit :: build (locals_are_invalidated_at_exit , body , move_data ,) , } ; for (block , block_data) in traversal :: preorder (body) { visitor . visit_basic_block_data (block , block_data) ; } BorrowSet { location_map : visitor . location_map , activation_map : visitor . activation_map , local_map : visitor . local_map , locals_state_at_exit : visitor . locals_state_at_exit , } } pub (crate) fn activations_at_location (& self , location : Location) -> & [BorrowIndex] { self . activation_map . get (& location) . map_or (& [] , | activations | & activations [..]) } pub (crate) fn len (& self) -> usize { self . location_map . len () } pub (crate) fn indices (& self) -> impl Iterator < Item = BorrowIndex > { BorrowIndex :: ZERO .. BorrowIndex :: from_usize (self . len ()) } pub (crate) fn iter_enumerated (& self) -> impl Iterator < Item = (BorrowIndex , & BorrowData < 'tcx >) > { self . indices () . zip (self . location_map . values ()) } pub (crate) fn get_index_of (& self , location : & Location) -> Option < BorrowIndex > { self . location_map . get_index_of (location) . map (BorrowIndex :: from) } }
};
}
