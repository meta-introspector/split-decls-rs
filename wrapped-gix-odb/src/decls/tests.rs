macro_rules! deps {
    () => {
        PackId!();
    };
}

macro_rules! tests {
    () => {
        deps!();
        # [cfg (test)] mod tests { use super :: * ; mod pack_id { use super :: PackId ; # [test] fn to_intrinsic_roundtrip () { let single = PackId { index : (1 << 15) - 1 , multipack_index : None , } ; let multi = PackId { index : (1 << 15) - 1 , multipack_index : Some ((1 << 16) - 1) , } ; assert_eq ! (PackId :: from_intrinsic_pack_id (single . to_intrinsic_pack_id ()) , single) ; assert_eq ! (PackId :: from_intrinsic_pack_id (multi . to_intrinsic_pack_id ()) , multi) ; } # [test] # [should_panic] fn max_supported_index_count () { PackId { index : 1 << 15 , multipack_index : None , } . to_intrinsic_pack_id () ; } } }
    };
}

tests!();