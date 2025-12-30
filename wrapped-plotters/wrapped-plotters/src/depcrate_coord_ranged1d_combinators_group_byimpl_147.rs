// Generated macro for impl_147 (impl)
macro_rules! Depcrate_coord_ranged1d_combinators_group_byimpl_147 {
() => {
// Module: crate::coord::ranged1d::combinators::group_by
// Provides: {"impl_147"}
// Dependencies: {}
impl < T : DiscreteRanged > Ranged for GroupBy < T > { type FormatOption = NoDefaultFormatting ; type ValueType = T :: ValueType ; fn map (& self , value : & T :: ValueType , limit : (i32 , i32)) -> i32 { self . 0 . map (value , limit) } fn range (& self) -> Range < T :: ValueType > { self . 0 . range () } fn key_points < HintType : KeyPointHint > (& self , hint : HintType) -> Vec < T :: ValueType > { let range = 0 .. (self . 0 . size () + self . 1) / self . 1 ; let interval = ((range . end - range . start + hint . bold_points () - 1) / hint . bold_points ()) . max (1) ; let count = (range . end - range . start) / interval ; let idx_iter = (0 .. hint . bold_points ()) . map (| x | x * interval) ; if hint . weight () . allow_light_points () && count < hint . bold_points () * 2 { let outer_ticks = idx_iter ; let outer_tick_size = interval * self . 1 ; let inner_ticks_per_group = hint . max_num_points () / outer_ticks . len () ; let inner_ticks = (outer_tick_size + inner_ticks_per_group - 1) / inner_ticks_per_group ; let inner_ticks : Vec < _ > = (0 .. (outer_tick_size / inner_ticks)) . map (move | x | x * inner_ticks) . collect () ; let size = self . 0 . size () ; return outer_ticks . flat_map (| base | inner_ticks . iter () . map (move | & ofs | base * self . 1 + ofs)) . take_while (| & idx | idx < size) . map (| x | self . 0 . from_index (x) . unwrap ()) . collect () ; } idx_iter . map (| x | self . 0 . from_index (x * self . 1) . unwrap ()) . collect () } }
};
}
