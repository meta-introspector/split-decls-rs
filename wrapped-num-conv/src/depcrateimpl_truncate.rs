// Generated macro for impl_truncate (macro)
macro_rules! Depcrateimpl_truncate {
() => {
// Module: crate
// Provides: {"impl_truncate"}
// Dependencies: {}
macro_rules ! impl_truncate { ($ ($ ($ from : ty) ,+ => $ to : ty ;) *) => { $ ($ (const _ : () = assert ! (core :: mem :: size_of ::<$ from > () >= core :: mem :: size_of ::<$ to > () , concat ! ("cannot truncate " , stringify ! ($ from) , " to " , stringify ! ($ to) , " because " , stringify ! ($ from) , " is smaller than " , stringify ! ($ to))) ; impl sealed :: TruncateTargetSealed <$ to > for $ from { fn truncate (self) -> $ to { self as _ } } impl TruncateTarget <$ to > for $ from { }) +) * } ; }
};
}
