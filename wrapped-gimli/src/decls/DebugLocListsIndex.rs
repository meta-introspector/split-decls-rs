macro_rules! DebugLocListsIndex {
    () => {
        # [doc = " An index into a set of location list offsets in the `.debug_loclists` section."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct DebugLocListsIndex < T = usize > (pub T) ;
    };
}

DebugLocListsIndex!();