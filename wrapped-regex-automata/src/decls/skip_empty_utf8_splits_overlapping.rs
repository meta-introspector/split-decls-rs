macro_rules! deps {
    () => {
        Input!();
        NFA!();
        OverlappingState!();
        MatchError!();
    };
}

macro_rules! skip_empty_utf8_splits_overlapping {
    () => {
        deps!();
        # [doc = " Runs the given overlapping `search` function (forwards or backwards) until"] # [doc = " a match is found whose offset does not split a codepoint."] # [doc = ""] # [doc = " This is *not* always correct to call. It should only be called when the"] # [doc = " underlying NFA has UTF-8 mode enabled *and* it can produce zero-width"] # [doc = " matches. Calling this when both of those things aren't true might result"] # [doc = " in legitimate matches getting skipped."] # [cold] # [inline (never)] fn skip_empty_utf8_splits_overlapping < F > (input : & Input < '_ > , state : & mut OverlappingState , mut search : F ,) -> Result < () , MatchError > where F : FnMut (& Input < '_ > , & mut OverlappingState) -> Result < () , MatchError > , { let mut hm = match state . get_match () { None => return Ok (()) , Some (hm) => hm , } ; if input . get_anchored () . is_anchored () { if ! input . is_char_boundary (hm . offset ()) { state . mat = None ; } return Ok (()) ; } while ! input . is_char_boundary (hm . offset ()) { search (input , state) ? ; hm = match state . get_match () { None => return Ok (()) , Some (hm) => hm , } ; } Ok (()) }
    };
}

skip_empty_utf8_splits_overlapping!();