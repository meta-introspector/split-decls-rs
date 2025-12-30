// Generated macro for SectionEntry (enum)
macro_rules! DepcrateSectionEntry {
() => {
// Module: crate
// Provides: {"SectionEntry"}
// Dependencies: {}
# [doc = " A view into an `Ini`, which may either be vacant or occupied."] pub enum SectionEntry < 'a > { Vacant (SectionVacantEntry < 'a >) , Occupied (SectionOccupiedEntry < 'a >) , }
};
}
