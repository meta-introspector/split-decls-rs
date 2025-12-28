macro_rules! find_duplicates {
    () => {
        # [doc = " Find duplicates in a sorted array."] # [doc = ""] # [doc = " The algorithm is simple: the array is sorted, duplicates"] # [doc = " must be placed next to each other, we can check only adjacent elements."] fn find_duplicates < T : PartialEq > (slice : & [T]) -> impl Iterator < Item = (& T , & T) > { slice . windows (2) . filter_map (| w | { if w [0] == w [1] { Some ((& w [0] , & w [1])) } else { None } }) }
    };
}

find_duplicates!();