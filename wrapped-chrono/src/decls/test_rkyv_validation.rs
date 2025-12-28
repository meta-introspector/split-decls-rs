macro_rules! deps {
    () => {
        NaiveTime!();
    };
}

macro_rules! test_rkyv_validation {
    () => {
        deps!();
        # [test] # [cfg (feature = "rkyv-validation")] fn test_rkyv_validation () { let t_min = NaiveTime :: MIN ; let bytes = rkyv :: to_bytes :: < _ , 8 > (& t_min) . unwrap () ; assert_eq ! (rkyv :: from_bytes ::< NaiveTime > (& bytes) . unwrap () , t_min) ; let t_max = NaiveTime :: MAX ; let bytes = rkyv :: to_bytes :: < _ , 8 > (& t_max) . unwrap () ; assert_eq ! (rkyv :: from_bytes ::< NaiveTime > (& bytes) . unwrap () , t_max) ; }
    };
}

test_rkyv_validation!();