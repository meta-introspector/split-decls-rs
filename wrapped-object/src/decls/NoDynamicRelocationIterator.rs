macro_rules! NoDynamicRelocationIterator {
    () => {
        # [doc = " An iterator for files that don't have dynamic relocations."] # [derive (Debug)] pub struct NoDynamicRelocationIterator ;
    };
}

NoDynamicRelocationIterator!()