macro_rules! deps {
    () => {
        MatchError!();
        Input!();
        Automaton!();
        FindIter!();
        Match!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < 'a , 'h , A : Automaton > FindIter < 'a , 'h , A > { # [doc = " Creates a new non-overlapping iterator. If the given automaton would"] # [doc = " return an error on a search with the given input configuration, then"] # [doc = " that error is returned here."] fn new (aut : & 'a A , input : Input < 'h > ,) -> Result < FindIter < 'a , 'h , A > , MatchError > { let _ = aut . start_state (input . get_anchored ()) ? ; Ok (FindIter { aut , input , last_match_end : None }) } # [doc = " Executes a search and returns a match if one is found."] # [doc = ""] # [doc = " This does not advance the input forward. It just executes a search"] # [doc = " based on the current configuration/offsets."] fn search (& self) -> Option < Match > { self . aut . try_find (& self . input) . expect ("already checked that no match error can occur") } # [doc = " Handles the special case of an empty match by ensuring that 1) the"] # [doc = " iterator always advances and 2) empty matches never overlap with other"] # [doc = " matches."] # [doc = ""] # [doc = " (1) is necessary because we principally make progress by setting the"] # [doc = " starting location of the next search to the ending location of the last"] # [doc = " match. But if a match is empty, then this results in a search that does"] # [doc = " not advance and thus does not terminate."] # [doc = ""] # [doc = " (2) is not strictly necessary, but makes intuitive sense and matches"] # [doc = " the presiding behavior of most general purpose regex engines."] # [doc = " (Obviously this crate isn't a regex engine, but we choose to match"] # [doc = " their semantics.) The \"intuitive sense\" here is that we want to report"] # [doc = " NON-overlapping matches. So for example, given the patterns 'a' and"] # [doc = " '' (an empty string) against the haystack 'a', without the special"] # [doc = " handling, you'd get the matches [0, 1) and [1, 1), where the latter"] # [doc = " overlaps with the end bounds of the former."] # [doc = ""] # [doc = " Note that we mark this cold and forcefully prevent inlining because"] # [doc = " handling empty matches like this is extremely rare and does require"] # [doc = " quite a bit of code, comparatively. Keeping this code out of the main"] # [doc = " iterator function keeps it smaller and more amenable to inlining"] # [doc = " itself."] # [cold] # [inline (never)] fn handle_overlapping_empty_match (& mut self , mut m : Match ,) -> Option < Match > { assert ! (m . is_empty ()) ; if Some (m . end ()) == self . last_match_end { self . input . set_start (self . input . start () . checked_add (1) . unwrap ()) ; m = self . search () ? ; } Some (m) } }
    };
}

impl_38!()