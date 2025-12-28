macro_rules! deps {
    () => {
        ParallelExtend!();
    };
}

macro_rules! check_extend_items {
    () => {
        deps!();
        # [test] fn check_extend_items () { fn check < C > () where C : Default + Eq + Debug + Extend < i32 > + for < 'a > Extend < & 'a i32 > + ParallelExtend < i32 > + for < 'a > ParallelExtend < & 'a i32 > , { let mut serial = C :: default () ; let mut parallel = C :: default () ; let v : Vec < _ > = (0 .. 128) . collect () ; serial . extend (& v) ; parallel . par_extend (& v) ; assert_eq ! (serial , parallel) ; serial . extend (- 128 .. 0) ; parallel . par_extend (- 128 .. 0) ; assert_eq ! (serial , parallel) ; } check :: < BTreeSet < _ > > () ; check :: < HashSet < _ > > () ; check :: < LinkedList < _ > > () ; check :: < Vec < _ > > () ; check :: < VecDeque < _ > > () ; }
    };
}

check_extend_items!();