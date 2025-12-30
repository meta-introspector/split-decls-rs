// Generated macro for impl_42 (impl)
macro_rules! Depcrateimpl_42 {
() => {
// Module: crate
// Provides: {"impl_42"}
// Dependencies: {}
impl < 'a > From < Entry < 'a , SectionKey , Properties > > for SectionEntry < 'a > { fn from (e : Entry < 'a , SectionKey , Properties >) -> SectionEntry < 'a > { match e { Entry :: Occupied (inner) => SectionEntry :: Occupied (SectionOccupiedEntry { inner }) , Entry :: Vacant (inner) => SectionEntry :: Vacant (SectionVacantEntry { inner }) , } } }
};
}
