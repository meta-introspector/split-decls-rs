macro_rules! deps {
    () => {
        Input!();
        MatchError!();
        MatchErrorKind!();
    };
}

macro_rules! tests {
    () => {
        deps!();
        # [cfg (test)] mod tests { use super :: * ; # [test] fn match_error_size () { let expected_size = if cfg ! (feature = "alloc") { core :: mem :: size_of :: < usize > () } else { 2 * core :: mem :: size_of :: < usize > () } ; assert_eq ! (expected_size , core :: mem :: size_of ::< MatchError > ()) ; } # [cfg (target_pointer_width = "64")] # [test] fn match_error_kind_size () { let expected_size = 2 * core :: mem :: size_of :: < usize > () ; assert_eq ! (expected_size , core :: mem :: size_of ::< MatchErrorKind > ()) ; } # [cfg (target_pointer_width = "32")] # [test] fn match_error_kind_size () { let expected_size = 3 * core :: mem :: size_of :: < usize > () ; assert_eq ! (expected_size , core :: mem :: size_of ::< MatchErrorKind > ()) ; } # [test] fn incorrect_asref_guard () { struct Bad (std :: cell :: Cell < bool >) ; impl AsRef < [u8] > for Bad { fn as_ref (& self) -> & [u8] { if self . 0 . replace (false) { & [] } else { & [0 ; 1000] } } } let bad = Bad (std :: cell :: Cell :: new (true)) ; let input = Input :: new (& bad) ; assert ! (input . end () <= input . haystack () . len ()) ; } }
    };
}

tests!();