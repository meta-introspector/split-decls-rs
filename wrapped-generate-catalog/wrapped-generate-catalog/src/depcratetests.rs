// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: Parameters ; # [test] fn it_parses_parameters () { let s = "width=3  poly=0x3  init=0x0  refin=false  refout=false  xorout=0x7  check=0x4  residue=0x2  name=\"CRC-3/GSM\"" ; let _params : Parameters = s . parse () . unwrap () ; } }
};
}
