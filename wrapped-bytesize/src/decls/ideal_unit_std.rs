macro_rules! ideal_unit_std {
    () => {
        # [cfg (feature = "std")] # [allow (dead_code)] fn ideal_unit_std (size : f64 , unit_base : f64) -> usize { assert ! (size . ln () >= unit_base , "only called when bytes >= unit") ; match (size . ln () / unit_base) as usize { 0 => unreachable ! () , e => e , } }
    };
}

ideal_unit_std!()