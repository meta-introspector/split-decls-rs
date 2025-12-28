macro_rules! deps {
    () => {
        IndexSet!();
        IndexMap!();
    };
}

macro_rules! borsh_tests {
    () => {
        deps!();
        # [cfg (test)] mod borsh_tests { use super :: * ; # [test] fn map_borsh_roundtrip () { let original_map : IndexMap < i32 , i32 > = { let mut map = IndexMap :: new () ; map . insert (1 , 2) ; map . insert (3 , 4) ; map . insert (5 , 6) ; map } ; let serialized_map = borsh :: to_vec (& original_map) . unwrap () ; let deserialized_map : IndexMap < i32 , i32 > = BorshDeserialize :: try_from_slice (& serialized_map) . unwrap () ; assert_eq ! (original_map , deserialized_map) ; } # [test] fn set_borsh_roundtrip () { let original_map : IndexSet < i32 > = [1 , 2 , 3 , 4 , 5 , 6] . into_iter () . collect () ; let serialized_map = borsh :: to_vec (& original_map) . unwrap () ; let deserialized_map : IndexSet < i32 > = BorshDeserialize :: try_from_slice (& serialized_map) . unwrap () ; assert_eq ! (original_map , deserialized_map) ; } }
    };
}

borsh_tests!()