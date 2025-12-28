macro_rules! deps {
    () => {
        DnfExpr!();
        CfgExpr!();
    };
}

macro_rules! proptest {
    () => {
        deps!();
        # [test] fn proptest () { const REPEATS : usize = 512 ; let mut rng = oorandom :: Rand32 :: new (123456789) ; let mut buf = Vec :: new () ; for _ in 0 .. REPEATS { buf . clear () ; while buf . len () < 512 { buf . extend (rng . rand_u32 () . to_ne_bytes ()) ; } let mut u = Unstructured :: new (& buf) ; let cfg = CfgExpr :: arbitrary (& mut u) . unwrap () ; DnfExpr :: new (& cfg) ; } }
    };
}

proptest!()