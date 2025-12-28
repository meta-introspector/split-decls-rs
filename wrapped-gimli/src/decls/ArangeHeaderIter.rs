macro_rules! deps {
    () => {
        Reader!();
        DebugArangesOffset!();
    };
}

macro_rules! ArangeHeaderIter {
    () => {
        deps!();
        # [doc = " An iterator over the headers of a `.debug_aranges` section."] # [derive (Clone , Debug)] pub struct ArangeHeaderIter < R : Reader > { input : R , offset : DebugArangesOffset < R :: Offset > , }
    };
}

ArangeHeaderIter!();