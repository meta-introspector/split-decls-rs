macro_rules! deps {
    () => {
        Diff!();
        Range!();
        Solution!();
    };
}

macro_rules! main {
    () => {
        deps!();
        fn main < 'a , 'b > (mut text1 : Range < 'a > , mut text2 : Range < 'b >) -> Solution < 'a , 'b > { let whole1 = text1 ; let whole2 = text2 ; let common_prefix_len = common_prefix (text1 , text2) ; let common_prefix = Diff :: Equal (text1 . substring (.. common_prefix_len) , text2 . substring (.. common_prefix_len) ,) ; text1 = text1 . substring (common_prefix_len ..) ; text2 = text2 . substring (common_prefix_len ..) ; let common_suffix_len = common_suffix (text1 , text2) ; let common_suffix = Diff :: Equal (text1 . substring (text1 . len - common_suffix_len ..) , text2 . substring (text2 . len - common_suffix_len ..) ,) ; text1 = text1 . substring (.. text1 . len - common_suffix_len) ; text2 = text2 . substring (.. text2 . len - common_suffix_len) ; let mut solution = Solution { text1 : whole1 , text2 : whole2 , diffs : compute (text1 , text2) , } ; if common_prefix_len > 0 { solution . diffs . insert (0 , common_prefix) ; } if common_suffix_len > 0 { solution . diffs . push (common_suffix) ; } cleanup_merge (& mut solution) ; solution }
    };
}

main!()