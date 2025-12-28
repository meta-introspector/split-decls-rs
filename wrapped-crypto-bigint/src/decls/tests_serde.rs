macro_rules! deps {
    () => {
        NonZero!();
    };
}

macro_rules! tests_serde {
    () => {
        deps!();
        # [cfg (all (test , feature = "serde"))] # [allow (clippy :: unwrap_used)] mod tests_serde { use crate :: { NonZero , U64 } ; # [test] fn serde () { let test = Option :: < NonZero < U64 > > :: from (NonZero :: new (U64 :: from_u64 (0x0011223344556677))) . unwrap () ; let serialized = bincode :: serde :: encode_to_vec (test , bincode :: config :: standard ()) . unwrap () ; let deserialized : NonZero < U64 > = bincode :: serde :: decode_from_slice (& serialized , bincode :: config :: standard ()) . unwrap () . 0 ; assert_eq ! (test , deserialized) ; let serialized = bincode :: serde :: encode_to_vec (U64 :: ZERO , bincode :: config :: standard ()) . unwrap () ; assert ! (bincode :: serde :: decode_from_slice ::< NonZero < U64 >, _ > (& serialized , bincode :: config :: standard ()) . is_err ()) ; } }
    };
}

tests_serde!();