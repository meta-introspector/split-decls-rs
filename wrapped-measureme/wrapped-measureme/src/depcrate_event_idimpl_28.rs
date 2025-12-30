// Generated macro for impl_28 (impl)
macro_rules! Depcrate_event_idimpl_28 {
() => {
// Module: crate::event_id
// Provides: {"impl_28"}
// Dependencies: {}
impl < 'p > EventIdBuilder < 'p > { pub fn new (profiler : & Profiler) -> EventIdBuilder < '_ > { EventIdBuilder { profiler } } # [inline] pub fn from_label (& self , label : StringId) -> EventId { EventId :: from_label (label) } pub fn from_label_and_arg (& self , label : StringId , arg : StringId) -> EventId { EventId (self . profiler . alloc_string (& [StringComponent :: Ref (label) , StringComponent :: Value (SEPARATOR_BYTE) , StringComponent :: Ref (arg) ,])) } pub fn from_label_and_args (& self , label : StringId , args : & [StringId]) -> EventId { let mut parts = SmallVec :: < StringComponent < '_ > , 7 > :: with_capacity (1 + args . len () * 2) ; parts . push (StringComponent :: Ref (label)) ; for arg in args { parts . push (StringComponent :: Value (SEPARATOR_BYTE)) ; parts . push (StringComponent :: Ref (* arg)) ; } EventId (self . profiler . alloc_string (& parts [..])) } }
};
}
