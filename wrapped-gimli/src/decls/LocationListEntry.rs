macro_rules! deps {
    () => {
        Range!();
        Reader!();
        Expression!();
    };
}

macro_rules! LocationListEntry {
    () => {
        deps!();
        # [doc = " A location list entry from the `.debug_loc` or `.debug_loclists` sections."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct LocationListEntry < R : Reader > { # [doc = " The address range that this location is valid for."] pub range : Range , # [doc = " The data containing a single location description."] pub data : Expression < R > , }
    };
}

LocationListEntry!()