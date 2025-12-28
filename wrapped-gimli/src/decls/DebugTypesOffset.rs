macro_rules! DebugTypesOffset {
    () => {
        # [doc = " An offset into the `.debug_types` section."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Ord , PartialOrd , Hash)] pub struct DebugTypesOffset < T = usize > (pub T) ;
    };
}

DebugTypesOffset!();