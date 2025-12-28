macro_rules! DebugRngListsIndex {
    () => {
        # [doc = " An index into a set of range list offsets in the `.debug_rnglists` section."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct DebugRngListsIndex < T = usize > (pub T) ;
    };
}

DebugRngListsIndex!();