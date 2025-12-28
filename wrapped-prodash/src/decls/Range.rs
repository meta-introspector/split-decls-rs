macro_rules! Range {
    () => {
        # [doc = " A helper for formatting numbers representing ranges in renderers as in `2 of 5 steps`."] # [derive (Copy , Clone , Default , Eq , PartialEq , Ord , PartialOrd , Debug)] pub struct Range { # [doc = " The name of the unit to be appended to the range."] pub name : & 'static str , }
    };
}

Range!();