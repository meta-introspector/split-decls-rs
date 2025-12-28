macro_rules! deps {
    () => {
        CharWindows!();
    };
}

macro_rules! str_windows {
    () => {
        deps!();
        # [test] fn str_windows () { assert_eq ! (CharWindows :: new ("abc" , 4) . next () , None) ; assert_eq ! (CharWindows :: new ("abc" , 3) . next () , Some ("abc")) ; assert_eq ! (CharWindows :: new ("abc" , 3) . count () , 1) ; assert_eq ! (CharWindows :: new ("αbγ" , 2) . nth (0) , Some ("αb")) ; assert_eq ! (CharWindows :: new ("αbγ" , 2) . nth (1) , Some ("bγ")) ; }
    };
}

str_windows!()