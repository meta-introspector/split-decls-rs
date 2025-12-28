macro_rules! deps {
    () => {
        Pathspec!();
        State!();
        Item!();
        Note!();
        Entry!();
    };
}

macro_rules! impl_250 {
    () => {
        deps!();
        # [doc = " Access"] impl < 'repo > Pathspec < 'repo > { # [doc = " Return the attributes cache which is used when matching attributes in pathspecs, or `None` if none of the pathspecs require that."] pub fn attributes (& self) -> Option < & gix_worktree :: Stack > { self . stack . as_ref () } # [doc = " Return the search itself which can be used for matching paths or accessing the actual patterns that will be used."] pub fn search (& self) -> & gix_pathspec :: Search { & self . search } # [doc = " Return the first [`Match`](search::Match) of `relative_path`, or `None`."] # [doc = " Note that the match might [be excluded](search::Match::is_excluded())."] # [doc = " `is_dir` is true if `relative_path` is a directory."] # [doc (alias = "match_diff" , alias = "match_tree" , alias = "match_index" , alias = "match_workdir" , alias = "matches_path" , alias = "git2")] pub fn pattern_matching_relative_path < 'a > (& mut self , relative_path : impl Into < & 'a BStr > , is_dir : Option < bool > ,) -> Option < gix_pathspec :: search :: Match < '_ > > { self . search . pattern_matching_relative_path (relative_path . into () , is_dir , & mut | relative_path , case , is_dir , out | { let stack = self . stack . as_mut () . expect ("initialized in advance") ; stack . set_case (case) . at_entry (relative_path , Some (is_dir_to_mode (is_dir)) , & self . repo . objects) . is_ok_and (| platform | platform . matching_attributes (out)) } ,) } # [doc = " The simplified version of [`pattern_matching_relative_path()`](Self::pattern_matching_relative_path()) which returns"] # [doc = " `true` if `relative_path` is included in the set of positive pathspecs, while not being excluded."] pub fn is_included < 'a > (& mut self , relative_path : impl Into < & 'a BStr > , is_dir : Option < bool >) -> bool { self . pattern_matching_relative_path (relative_path , is_dir) . is_some_and (| m | ! m . is_excluded ()) } # [doc = " Return an iterator over all entries along with their path if the path matches the pathspec, or `None` if the pathspec is"] # [doc = " known to match no entry."] pub fn index_entries_with_paths < 's : 'repo , 'a : 'repo > (& 's mut self , index : & 'a gix_index :: State ,) -> Option < impl Iterator < Item = (& 'a BStr , & 'a gix_index :: Entry) > + 'repo + 's > { index . prefixed_entries (self . search . common_prefix ()) . map (| entries | { entries . iter () . filter_map (move | entry | { let path = entry . path (index) ; self . is_included (path , Some (false)) . then_some ((path , entry)) }) }) } }
    };
}

impl_250!()