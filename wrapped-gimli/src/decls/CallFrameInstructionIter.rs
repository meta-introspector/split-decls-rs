macro_rules! deps {
    () => {
        PointerEncodingParameters!();
        Vendor!();
        Reader!();
    };
}

macro_rules! CallFrameInstructionIter {
    () => {
        deps!();
        # [doc = " A lazy iterator parsing call frame instructions."] # [doc = ""] # [doc = " Can be [used with"] # [doc = " `FallibleIterator`](./index.html#using-with-fallibleiterator)."] # [derive (Clone , Debug)] pub struct CallFrameInstructionIter < 'a , R : Reader > { input : R , address_encoding : Option < constants :: DwEhPe > , parameters : PointerEncodingParameters < 'a , R > , vendor : Vendor , }
    };
}

CallFrameInstructionIter!()