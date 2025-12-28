macro_rules! test_slice {
    () => {
        # [allow (deprecated)] # [test] fn test_slice () { let t = "αβγabc" ; assert_eq ! (t . get_slice (..) , Some (t)) ; assert_eq ! (t . get_slice (0 .. t . len ()) , Some (t)) ; assert_eq ! (t . get_slice (1 ..) , None) ; assert_eq ! (t . get_slice (0 .. t . len () + 1) , None) ; assert_eq ! (t . get_slice (t . len () + 1 ..) , None) ; assert_eq ! (t . get_slice (t . len () ..) , Some ("")) ; }
    };
}

test_slice!()