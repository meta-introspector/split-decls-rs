macro_rules! deps {
    () => {
        ParallelExtend!();
    };
}

macro_rules! check_extend_pairs {
    () => {
        deps!();
        # [test] fn check_extend_pairs () { fn check < C > () where C : Default + Eq + Debug + Extend < (usize , i32) > + for < 'a > Extend < (& 'a usize , & 'a i32) > + ParallelExtend < (usize , i32) > + for < 'a > ParallelExtend < (& 'a usize , & 'a i32) > , { let mut serial = C :: default () ; let mut parallel = C :: default () ; let m : HashMap < _ , _ > = (0 .. 128) . enumerate () . collect () ; serial . extend (& m) ; parallel . par_extend (& m) ; assert_eq ! (serial , parallel) ; let v : Vec < (_ , _) > = (- 128 .. 0) . enumerate () . collect () ; serial . extend (v . clone ()) ; parallel . par_extend (v) ; assert_eq ! (serial , parallel) ; } check :: < BTreeMap < usize , i32 > > () ; check :: < HashMap < usize , i32 > > () ; }
    };
}

check_extend_pairs!();