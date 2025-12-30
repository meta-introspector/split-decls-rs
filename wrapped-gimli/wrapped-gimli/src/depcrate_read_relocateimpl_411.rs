// Generated macro for impl_411 (impl)
macro_rules! Depcrate_read_relocateimpl_411 {
() => {
// Module: crate::read::relocate
// Provides: {"impl_411"}
// Dependencies: {}
impl < R , T > RelocateReader < R , T > where R : Reader < Offset = usize > , T : Relocate < R :: Offset > , { # [doc = " Create a new `RelocateReader` which applies relocations to the given section reader."] pub fn new (section : R , relocate : T) -> Self { let reader = section . clone () ; Self { section , reader , relocate , } } }
};
}
