macro_rules! deps {
    () => {
        Range!();
    };
}

macro_rules! common_overlap {
    () => {
        deps!();
        fn common_overlap (mut text1 : Range , mut text2 : Range) -> usize { if text1 . is_empty () || text2 . is_empty () { return 0 ; } if text1 . len > text2 . len { text1 = text1 . substring (text1 . len - text2 . len ..) ; } else if text1 . len < text2 . len { text2 = text2 . substring (.. text1 . len) ; } if slice (text1) == slice (text2) { return text1 . len ; } let mut best = 0 ; let mut length = 1 ; loop { let pattern = text1 . substring (text1 . len - length ..) ; let found = match text2 . find (pattern) { Some (found) => found , None => return best , } ; length += found ; if found == 0 || slice (text1 . substring (text1 . len - length ..)) == slice (text2 . substring (.. length)) { best = length ; length += 1 ; } } }
    };
}

common_overlap!();