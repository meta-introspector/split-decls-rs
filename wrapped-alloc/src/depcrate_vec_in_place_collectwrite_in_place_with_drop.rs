// Generated macro for write_in_place_with_drop (function)
macro_rules! Depcrate_vec_in_place_collectwrite_in_place_with_drop {
() => {
// Module: crate::vec::in_place_collect
// Provides: {"write_in_place_with_drop"}
// Dependencies: {}
fn write_in_place_with_drop < T > (src_end : * const T ,) -> impl FnMut (InPlaceDrop < T > , T) -> Result < InPlaceDrop < T > , ! > { move | mut sink , item | { unsafe { debug_assert ! (sink . dst as * const _ <= src_end , "InPlaceIterable contract violation") ; ptr :: write (sink . dst , item) ; sink . dst = sink . dst . add (1) ; } Ok (sink) } }
};
}
