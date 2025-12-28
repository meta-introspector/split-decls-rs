macro_rules! deps {
    () => {
        ImportMapIndex!();
        ItemInNs!();
    };
}

macro_rules! ImportMap {
    () => {
        deps!();
        # [doc = " A map from publicly exported items to its name."] # [doc = ""] # [doc = " Reexports of items are taken into account."] # [derive (Default)] pub struct ImportMap { # [doc = " Maps from `ItemInNs` to information of imports that bring the item into scope."] item_to_info_map : ImportMapIndex , # [doc = " List of keys stored in [`Self::item_to_info_map`], sorted lexicographically by their"] # [doc = " [`Name`]. Indexed by the values returned by running `fst`."] # [doc = ""] # [doc = " Since a name can refer to multiple items due to namespacing and import aliases, we store all"] # [doc = " items with the same name right after each other. This allows us to find all items after the"] # [doc = " fst gives us the index of the first one."] # [doc = ""] # [doc = " The [`u32`] is the index into the smallvec in the value of [`Self::item_to_info_map`]."] importables : Vec < (ItemInNs , u32) > , fst : fst :: Map < Vec < u8 > > , }
    };
}

ImportMap!();