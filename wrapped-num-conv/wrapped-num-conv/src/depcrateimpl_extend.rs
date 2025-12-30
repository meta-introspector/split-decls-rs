// Generated macro for impl_extend (macro)
macro_rules! Depcrateimpl_extend {
() => {
// Module: crate
// Provides: {"impl_extend"}
// Dependencies: {}
macro_rules ! impl_extend { ($ ($ from : ty => $ ($ to : ty) ,+;) *) => { $ ($ (const _ : () = assert ! (core :: mem :: size_of ::<$ from > () <= core :: mem :: size_of ::<$ to > () , concat ! ("cannot extend " , stringify ! ($ from) , " to " , stringify ! ($ to) , " because " , stringify ! ($ from) , " is larger than " , stringify ! ($ to))) ; impl sealed :: ExtendTargetSealed <$ to > for $ from { fn extend (self) -> $ to { self as _ } } impl ExtendTarget <$ to > for $ from { }) +) * } ; }
};
}
