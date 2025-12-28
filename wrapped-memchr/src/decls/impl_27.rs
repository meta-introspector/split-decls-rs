macro_rules! deps {
    () => {
        Seed!();
        Test!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl Seed { # [doc = " Controls how much we expand the haystack on either side for each test."] # [doc = " We lower this on Miri because otherwise running the tests would take"] # [doc = " forever."] const EXPAND_LEN : usize = { # [cfg (not (miri))] { 515 } # [cfg (miri)] { 6 } } ; # [doc = " Expand this test into many variations of the same test."] # [doc = ""] # [doc = " In particular, this will generate more tests with larger corpus sizes."] # [doc = " The expected positions are updated to maintain the integrity of the"] # [doc = " test."] # [doc = ""] # [doc = " This is important in testing a memchr implementation, because there are"] # [doc = " often different cases depending on the length of the corpus."] # [doc = ""] # [doc = " Note that we extend the corpus by adding `%` bytes, which we"] # [doc = " don't otherwise use as a needle."] fn generate (& self) -> impl Iterator < Item = Test > { let mut more = vec ! [] ; for i in 0 .. Seed :: EXPAND_LEN { let mut t = Test :: new (self) ; let mut new : String = core :: iter :: repeat ('%') . take (i) . collect () ; new . push_str (& t . haystack) ; t . haystack = new ; t . expected = t . expected . into_iter () . map (| p | p + i) . collect () ; more . push (t) ; } for i in 1 .. Seed :: EXPAND_LEN { let mut t = Test :: new (self) ; let padding : String = core :: iter :: repeat ('%') . take (i) . collect () ; t . haystack . push_str (& padding) ; more . push (t) ; } more . into_iter () } }
    };
}

impl_27!()