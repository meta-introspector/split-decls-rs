macro_rules! deps {
    () => {
        DFA!();
        Match!();
        Input!();
        Anchored!();
        Regex!();
        Cache!();
        NFA!();
        MatchError!();
    };
}

macro_rules! impl_276 {
    () => {
        deps!();
        # [doc = " Lower level \"search\" primitives that accept a `&Input` for cheap reuse"] # [doc = " and return an error if one occurs instead of panicking."] impl Regex { # [doc = " Returns the start and end offset of the leftmost match. If no match"] # [doc = " exists, then `None` is returned."] # [doc = ""] # [doc = " This is like [`Regex::find`] but with two differences:"] # [doc = ""] # [doc = " 1. It is not generic over `Into<Input>` and instead accepts a"] # [doc = " `&Input`. This permits reusing the same `Input` for multiple searches"] # [doc = " without needing to create a new one. This _may_ help with latency."] # [doc = " 2. It returns an error if the search could not complete where as"] # [doc = " [`Regex::find`] will panic."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This routine errors if the search could not complete. This can occur"] # [doc = " in a number of circumstances:"] # [doc = ""] # [doc = " * The configuration of the lazy DFA may permit it to \"quit\" the search."] # [doc = " For example, setting quit bytes or enabling heuristic support for"] # [doc = " Unicode word boundaries. The default configuration does not enable any"] # [doc = " option that could result in the lazy DFA quitting."] # [doc = " * The configuration of the lazy DFA may also permit it to \"give up\""] # [doc = " on a search if it makes ineffective use of its transition table"] # [doc = " cache. The default configuration does not enable this by default,"] # [doc = " although it is typically a good idea to."] # [doc = " * When the provided `Input` configuration is not supported. For"] # [doc = " example, by providing an unsupported anchor mode."] # [doc = ""] # [doc = " When a search returns an error, callers cannot know whether a match"] # [doc = " exists or not."] # [inline] pub fn try_search (& self , cache : & mut Cache , input : & Input < '_ > ,) -> Result < Option < Match > , MatchError > { let (fcache , rcache) = (& mut cache . forward , & mut cache . reverse) ; let end = match self . forward () . try_search_fwd (fcache , input) ? { None => return Ok (None) , Some (end) => end , } ; if input . start () == end . offset () { return Ok (Some (Match :: new (end . pattern () , end . offset () .. end . offset () ,))) ; } if self . is_anchored (input) { return Ok (Some (Match :: new (end . pattern () , input . start () .. end . offset () ,))) ; } let revsearch = input . clone () . span (input . start () .. end . offset ()) . anchored (Anchored :: Yes) . earliest (false) ; let start = self . reverse () . try_search_rev (rcache , & revsearch) ? . expect ("reverse search must match if forward search does") ; debug_assert_eq ! (start . pattern () , end . pattern () , "forward and reverse search must match same pattern" ,) ; debug_assert ! (start . offset () <= end . offset ()) ; Ok (Some (Match :: new (end . pattern () , start . offset () .. end . offset ()))) } # [doc = " Returns true if either the given input specifies an anchored search"] # [doc = " or if the underlying NFA is always anchored."] fn is_anchored (& self , input : & Input < '_ >) -> bool { match input . get_anchored () { Anchored :: No => { self . forward () . get_nfa () . is_always_start_anchored () } Anchored :: Yes | Anchored :: Pattern (_) => true , } } }
    };
}

impl_276!()