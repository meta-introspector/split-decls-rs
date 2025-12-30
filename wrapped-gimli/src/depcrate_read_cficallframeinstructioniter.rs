// Generated macro for CallFrameInstructionIter (struct)
macro_rules! Depcrate_read_cfiCallFrameInstructionIter {
() => {
// Module: crate::read::cfi
// Provides: {"CallFrameInstructionIter"}
// Dependencies: {}
# [doc = " A lazy iterator parsing call frame instructions."] # [doc = ""] # [doc = " Can be [used with"] # [doc = " `FallibleIterator`](./index.html#using-with-fallibleiterator)."] # [derive (Clone , Debug)] pub struct CallFrameInstructionIter < 'a , R : Reader > { input : R , address_encoding : Option < constants :: DwEhPe > , parameters : PointerEncodingParameters < 'a , R > , vendor : Vendor , }
};
}
