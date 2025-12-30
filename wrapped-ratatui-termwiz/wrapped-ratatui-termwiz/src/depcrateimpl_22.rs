// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
impl FromTermwiz < Intensity > for Modifier { fn from_termwiz (value : Intensity) -> Self { match value { Intensity :: Normal => Self :: empty () , Intensity :: Bold => Self :: BOLD , Intensity :: Half => Self :: DIM , } } }
};
}
