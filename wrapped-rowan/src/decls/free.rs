macro_rules! deps {
    () => {
        Green!();
        NodeData!();
    };
}

macro_rules! free {
    () => {
        deps!();
        # [inline (never)] unsafe fn free (mut data : ptr :: NonNull < NodeData >) { unsafe { loop { debug_assert_eq ! (data . as_ref () . rc . get () , 0) ; debug_assert ! (data . as_ref () . first . get () . is_null ()) ; let node = Box :: from_raw (data . as_ptr ()) ; match node . parent . take () { Some (parent) => { debug_assert ! (parent . as_ref () . rc . get () > 0) ; if node . mutable { sll :: unlink (& parent . as_ref () . first , & * node) } if parent . as_ref () . dec_rc () { data = parent ; } else { break ; } } None => { match & node . green { Green :: Node { ptr } => { let _ = GreenNode :: from_raw (ptr . get ()) ; } Green :: Token { ptr } => { let _ = GreenToken :: from_raw (* ptr) ; } } break ; } } } } }
    };
}

free!();