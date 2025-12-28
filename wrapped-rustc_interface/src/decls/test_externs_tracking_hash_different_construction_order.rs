macro_rules! test_externs_tracking_hash_different_construction_order {
    () => {
        # [test] fn test_externs_tracking_hash_different_construction_order () { let mut v1 = Options :: default () ; let mut v2 = Options :: default () ; let mut v3 = Options :: default () ; v1 . externs = Externs :: new (mk_map (vec ! [(String :: from ("a") , new_public_extern_entry (vec ! ["b" , "c"])) , (String :: from ("d") , new_public_extern_entry (vec ! ["e" , "f"])) ,])) ; v2 . externs = Externs :: new (mk_map (vec ! [(String :: from ("d") , new_public_extern_entry (vec ! ["e" , "f"])) , (String :: from ("a") , new_public_extern_entry (vec ! ["b" , "c"])) ,])) ; v3 . externs = Externs :: new (mk_map (vec ! [(String :: from ("a") , new_public_extern_entry (vec ! ["b" , "c"])) , (String :: from ("d") , new_public_extern_entry (vec ! ["f" , "e"])) ,])) ; assert_same_hash (& v1 , & v2) ; assert_same_hash (& v1 , & v3) ; assert_same_hash (& v2 , & v3) ; }
    };
}

test_externs_tracking_hash_different_construction_order!()