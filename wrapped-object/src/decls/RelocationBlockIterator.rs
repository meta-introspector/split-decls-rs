macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! RelocationBlockIterator {
    () => {
        deps!();
        # [doc = " An iterator over the relocation blocks in the `.reloc` section of a PE file."] # [doc = ""] # [doc = " Returned by [`DataDirectories::relocation_blocks`](super::DataDirectories::relocation_blocks)."] # [derive (Debug , Default , Clone , Copy)] pub struct RelocationBlockIterator < 'data > { data : Bytes < 'data > , }
    };
}

RelocationBlockIterator!()