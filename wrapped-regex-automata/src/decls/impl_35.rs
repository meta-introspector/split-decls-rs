macro_rules! deps {
    () => {
        DFA!();
        StartTable!();
        BuildError!();
        StartKind!();
        StartByteMap!();
        Start!();
        PatternID!();
        LookMatcher!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        # [cfg (feature = "dfa-build")] impl StartTable < Vec < u32 > > { # [doc = " Create a valid set of start states all pointing to the dead state."] # [doc = ""] # [doc = " When the corresponding DFA is constructed with start states for each"] # [doc = " pattern, then `patterns` should be the number of patterns. Otherwise,"] # [doc = " it should be zero."] # [doc = ""] # [doc = " If the total table size could exceed the allocatable limit, then this"] # [doc = " returns an error. In practice, this is unlikely to be able to occur,"] # [doc = " since it's likely that allocation would have failed long before it got"] # [doc = " to this point."] fn dead (kind : StartKind , lookm : & LookMatcher , pattern_len : Option < usize > ,) -> Result < StartTable < Vec < u32 > > , BuildError > { if let Some (len) = pattern_len { assert ! (len <= PatternID :: LIMIT) ; } let stride = Start :: len () ; let starts_len = stride . checked_mul (2) . unwrap () ; let pattern_starts_len = match stride . checked_mul (pattern_len . unwrap_or (0)) { Some (x) => x , None => return Err (BuildError :: too_many_start_states ()) , } ; let table_len = match starts_len . checked_add (pattern_starts_len) { Some (x) => x , None => return Err (BuildError :: too_many_start_states ()) , } ; if let Err (_) = isize :: try_from (table_len) { return Err (BuildError :: too_many_start_states ()) ; } let table = vec ! [DEAD . as_u32 () ; table_len] ; let start_map = StartByteMap :: new (lookm) ; Ok (StartTable { table , kind , start_map , stride , pattern_len , universal_start_unanchored : None , universal_start_anchored : None , }) } }
    };
}

impl_35!()