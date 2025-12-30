// Generated macro for test_min_indentation (function)
macro_rules! Depcrate_snapshottest_min_indentation {
() => {
// Module: crate::snapshot
// Provides: {"test_min_indentation"}
// Dependencies: {}
# [test] fn test_min_indentation () { use similar_asserts :: assert_eq ; assert_eq ! (min_indentation (r#"
   1
   2
   "# ,) , "   " . to_string ()) ; assert_eq ! (min_indentation (r#"
            1
    2"#) , "    " . to_string ()) ; assert_eq ! (min_indentation (r#"
            1
            2
    "#) , "            " . to_string ()) ; assert_eq ! (min_indentation (r#"
   1
   2
"#) , "   " . to_string ()) ; assert_eq ! (min_indentation (r#"
        a
    "#) , "        " . to_string ()) ; assert_eq ! (min_indentation ("") , "" . to_string ()) ; assert_eq ! (min_indentation (r#"
    a
    b
c
    "#) , "" . to_string ()) ; assert_eq ! (min_indentation (r#"
a
    "#) , "" . to_string ()) ; assert_eq ! (min_indentation ("
    a") , "    " . to_string ()) ; assert_eq ! (min_indentation (r#"a
  a"#) , "" . to_string ()) ; assert_eq ! (normalize_inline (r#"
			1
	2"#) , r###"
		1
2"###) ; assert_eq ! (normalize_inline (r#"
	  	  1
	  	  2
    "#) , r###"
1
2
"###) ; }
};
}
