// Generated macro for impl_255 (impl)
macro_rules! Depcrate_coord_ranged1d_types_datetimeimpl_255 {
() => {
// Module: crate::coord::ranged1d::types::datetime
// Provides: {"impl_255"}
// Dependencies: {}
impl < T : TimeValue + Clone > Ranged for Monthly < T > where Range < T > : AsRangedCoord < Value = T > , { type FormatOption = NoDefaultFormatting ; type ValueType = T ; fn range (& self) -> Range < T > { self . 0 . start . clone () .. self . 0 . end . clone () } fn map (& self , value : & Self :: ValueType , limit : (i32 , i32)) -> i32 { T :: map_coord (value , & self . 0 . start , & self . 0 . end , limit) } fn key_points < HintType : KeyPointHint > (& self , hint : HintType) -> Vec < Self :: ValueType > { if hint . weight () . allow_light_points () && self . size () <= hint . bold_points () * 2 { let coord : < Range < T > as AsRangedCoord > :: CoordDescType = self . 0 . clone () . into () ; let normal = coord . key_points (hint . max_num_points ()) ; return normal ; } self . bold_key_points (& hint) } }
};
}
