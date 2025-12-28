macro_rules! LocationListsOffset {
    () => {
        # [doc = " An offset into either the `.debug_loc` section or the `.debug_loclists` section,"] # [doc = " depending on the version of the unit the offset was contained in."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct LocationListsOffset < T = usize > (pub T) ;
    };
}

LocationListsOffset!()