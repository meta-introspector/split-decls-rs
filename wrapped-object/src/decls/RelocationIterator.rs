macro_rules! deps {
    () => {
        U16!();
    };
}

macro_rules! RelocationIterator {
    () => {
        deps!();
        # [doc = " An iterator of the relocations in a block in the `.reloc` section of a PE file."] # [derive (Debug , Clone)] pub struct RelocationIterator < 'data > { virtual_address : u32 , size : u32 , relocs : slice :: Iter < 'data , U16 < LE > > , }
    };
}

RelocationIterator!()