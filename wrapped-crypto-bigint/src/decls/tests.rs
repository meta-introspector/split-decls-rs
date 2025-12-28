macro_rules! deps {
    () => {
        Wrapping!();
    };
}

macro_rules! tests {
    () => {
        deps!();
        # [cfg (all (test , feature = "serde"))] # [allow (clippy :: unwrap_used)] mod tests { use crate :: { U64 , Wrapping } ; # [test] fn serde () { const TEST : Wrapping < U64 > = Wrapping (U64 :: from_u64 (0x0011223344556677)) ; let serialized = bincode :: serde :: encode_to_vec (TEST , bincode :: config :: standard ()) . unwrap () ; let deserialized : Wrapping < U64 > = bincode :: serde :: decode_from_slice (& serialized , bincode :: config :: standard ()) . unwrap () . 0 ; assert_eq ! (TEST , deserialized) ; } }
    };
}

tests!();