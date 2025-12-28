macro_rules! DebugAbbrevOffset {
    () => {
        # [doc = " An offset into the `.debug_abbrev` section."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct DebugAbbrevOffset < T = usize > (pub T) ;
    };
}

DebugAbbrevOffset!()