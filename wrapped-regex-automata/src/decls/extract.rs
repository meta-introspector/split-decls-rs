macro_rules! deps {
    () => {
        Prefilter!();
    };
}

macro_rules! extract {
    () => {
        deps!();
        # [doc = " Attempts to extract an \"inner\" prefilter from the given HIR expressions. If"] # [doc = " one was found, then a concatenation of the HIR expressions that precede it"] # [doc = " is returned."] # [doc = ""] # [doc = " The idea here is that the prefilter returned can be used to find candidate"] # [doc = " matches. And then the HIR returned can be used to build a reverse regex"] # [doc = " matcher, which will find the start of the candidate match. Finally, the"] # [doc = " match still has to be confirmed with a normal anchored forward scan to find"] # [doc = " the end position of the match."] # [doc = ""] # [doc = " Note that this assumes leftmost-first match semantics, so callers must"] # [doc = " not call this otherwise."] pub (crate) fn extract (hirs : & [& Hir]) -> Option < (Hir , Prefilter) > { if hirs . len () != 1 { debug ! ("skipping reverse inner optimization since it only \
		 	 supports 1 pattern, {} were given" , hirs . len () ,) ; return None ; } let mut concat = match top_concat (hirs [0]) { Some (concat) => concat , None => { debug ! ("skipping reverse inner optimization because a top-level \
		 	     concatenation could not found" ,) ; return None ; } } ; for i in 1 .. concat . len () { let hir = & concat [i] ; let pre = match prefilter (hir) { None => continue , Some (pre) => pre , } ; if ! pre . is_fast () { debug ! ("skipping extracted inner prefilter because \
				 it probably isn't fast") ; continue ; } let concat_suffix = Hir :: concat (concat . split_off (i)) ; let concat_prefix = Hir :: concat (concat) ; let pre2 = match prefilter (& concat_suffix) { None => pre , Some (pre2) => { if pre2 . is_fast () { pre2 } else { pre } } } ; return Some ((concat_prefix , pre2)) ; } debug ! ("skipping reverse inner optimization because a top-level \
	     sub-expression with a fast prefilter could not be found") ; None }
    };
}

extract!()