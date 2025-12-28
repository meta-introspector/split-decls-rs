macro_rules! deps {
    () => {
        TransformTtAction!();
    };
}

macro_rules! transform_tt {
    () => {
        deps!();
        # [doc = " This function takes a token tree, and calls `callback` with each token tree in it."] # [doc = " Then it does what the callback says: keeps the tt or replaces it with a (possibly empty)"] # [doc = " tts view."] fn transform_tt < 'a , 'b > (tt : & 'a mut Vec < tt :: TokenTree > , mut callback : impl FnMut (& mut tt :: TokenTree) -> TransformTtAction < 'b > ,) { let mut subtrees_stack = Vec :: new () ; let mut i = 0 ; while i < tt . len () { 'pop_finished_subtrees : while let Some (& subtree_idx) = subtrees_stack . last () { let tt :: TokenTree :: Subtree (subtree) = & tt [subtree_idx] else { unreachable ! ("non-subtree on subtrees stack") ; } ; if i >= subtree_idx + 1 + subtree . usize_len () { subtrees_stack . pop () ; } else { break 'pop_finished_subtrees ; } } let action = callback (& mut tt [i]) ; match action { TransformTtAction :: Keep => { if let tt :: TokenTree :: Subtree (_) = & tt [i] { subtrees_stack . push (i) ; } i += 1 ; } TransformTtAction :: ReplaceWith (replacement) => { let old_len = 1 + match & tt [i] { tt :: TokenTree :: Leaf (_) => 0 , tt :: TokenTree :: Subtree (subtree) => subtree . usize_len () , } ; let len_diff = replacement . len () as i64 - old_len as i64 ; tt . splice (i .. i + old_len , replacement . flat_tokens () . iter () . cloned ()) ; i += replacement . len () ; for & subtree_idx in & subtrees_stack { let tt :: TokenTree :: Subtree (subtree) = & mut tt [subtree_idx] else { unreachable ! ("non-subtree on subtrees stack") ; } ; subtree . len = (i64 :: from (subtree . len) + len_diff) . try_into () . unwrap () ; } } } } }
    };
}

transform_tt!()