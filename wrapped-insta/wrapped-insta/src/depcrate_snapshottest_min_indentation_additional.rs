// Generated macro for test_min_indentation_additional (function)
macro_rules! Depcrate_snapshottest_min_indentation_additional {
() => {
// Module: crate::snapshot
// Provides: {"test_min_indentation_additional"}
// Dependencies: {}
# [test] fn test_min_indentation_additional () { use similar_asserts :: assert_eq ; let t = r#"
   1
   2
"# ; assert_eq ! (min_indentation (t) , "   " . to_string ()) ; let t = r#"
        a
    "# ; assert_eq ! (min_indentation (t) , "        " . to_string ()) ; let t = "" ; assert_eq ! (min_indentation (t) , "" . to_string ()) ; let t = r#"
    a
    b
c
    "# ; assert_eq ! (min_indentation (t) , "" . to_string ()) ; let t = r#"
a"# ; assert_eq ! (min_indentation (t) , "" . to_string ()) ; let t = r#"
    a"# ; assert_eq ! (min_indentation (t) , "    " . to_string ()) ; let t = r#"a
  a"# ; assert_eq ! (min_indentation (t) , "" . to_string ()) ; let t = r#"
 	1
 	2
    "# ; assert_eq ! (min_indentation (t) , " 	" . to_string ()) ; let t = r#"
  	  	  	1
  	2"# ; assert_eq ! (min_indentation (t) , "  	" . to_string ()) ; let t = r#"
			1
	2"# ; assert_eq ! (min_indentation (t) , "	" . to_string ()) ; }
};
}
