macro_rules! deps {
    () => {
        Literal!();
        HirKind!();
        Hir!();
        Concat!();
        Result!();
    };
}

macro_rules! lift_common_prefix {
    () => {
        deps!();
        # [doc = " Looks for a common prefix in the list of alternation branches given. If one"] # [doc = " is found, then an equivalent but (hopefully) simplified Hir is returned."] # [doc = " Otherwise, the original given list of branches is returned unmodified."] # [doc = ""] # [doc = " This is not quite as good as it could be. Right now, it requires that"] # [doc = " all branches are 'Concat' expressions. It also doesn't do well with"] # [doc = " literals. For example, given 'foofoo|foobar', it will not refactor it to"] # [doc = " 'foo(?:foo|bar)' because literals are flattened into their own special"] # [doc = " concatenation. (One wonders if perhaps 'Literal' should be a single atom"] # [doc = " instead of a string of bytes because of this. Otherwise, handling the"] # [doc = " current representation in this routine will be pretty gnarly. Sigh.)"] fn lift_common_prefix (hirs : Vec < Hir >) -> Result < Hir , Vec < Hir > > { if hirs . len () <= 1 { return Err (hirs) ; } let mut prefix = match hirs [0] . kind () { HirKind :: Concat (ref xs) => & * * xs , _ => return Err (hirs) , } ; if prefix . is_empty () { return Err (hirs) ; } for h in hirs . iter () . skip (1) { let concat = match h . kind () { HirKind :: Concat (ref xs) => xs , _ => return Err (hirs) , } ; let common_len = prefix . iter () . zip (concat . iter ()) . take_while (| (x , y) | x == y) . count () ; prefix = & prefix [.. common_len] ; if prefix . is_empty () { return Err (hirs) ; } } let len = prefix . len () ; assert_ne ! (0 , len) ; let mut prefix_concat = vec ! [] ; let mut suffix_alts = vec ! [] ; for h in hirs { let mut concat = match h . into_kind () { HirKind :: Concat (xs) => xs , _ => unreachable ! () , } ; suffix_alts . push (Hir :: concat (concat . split_off (len))) ; if prefix_concat . is_empty () { prefix_concat = concat ; } } let mut concat = prefix_concat ; concat . push (Hir :: alternation (suffix_alts)) ; Ok (Hir :: concat (concat)) }
    };
}

lift_common_prefix!();