// Generated macro for impl_126 (impl)
macro_rules! Depcrateimpl_126 {
() => {
// Module: crate
// Provides: {"impl_126"}
// Dependencies: {}
impl AbiAlign { # [inline] pub fn new (align : Align) -> AbiAlign { AbiAlign { abi : align } } # [inline] pub fn min (self , other : AbiAlign) -> AbiAlign { AbiAlign { abi : self . abi . min (other . abi) } } # [inline] pub fn max (self , other : AbiAlign) -> AbiAlign { AbiAlign { abi : self . abi . max (other . abi) } } }
};
}
