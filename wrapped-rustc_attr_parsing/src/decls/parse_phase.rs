macro_rules! deps {
    () => {
        AcceptContext!();
        Stage!();
    };
}

macro_rules! parse_phase {
    () => {
        deps!();
        fn parse_phase < S : Stage > (cx : & mut AcceptContext < '_ , '_ , S > , phase : Option < (Symbol , Span) > , failed : & mut bool ,) -> Option < (MirPhase , Span) > { let (phase , span) = phase ? ; let phase = match phase { sym :: initial => MirPhase :: Initial , sym :: post_cleanup => MirPhase :: PostCleanup , sym :: optimized => MirPhase :: Optimized , _ => { cx . expected_specific_argument (span , & [sym :: initial , sym :: post_cleanup , sym :: optimized]) ; * failed = true ; return None ; } } ; Some ((phase , span)) }
    };
}

parse_phase!()