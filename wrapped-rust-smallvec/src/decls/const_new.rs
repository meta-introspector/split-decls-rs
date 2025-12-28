macro_rules! const_new {
    () => {
        # [test] fn const_new () { let v = const_new_inner () ; assert_eq ! (v . capacity () , 4) ; assert_eq ! (v . len () , 0) ; let v = const_new_inline_sized () ; assert_eq ! (v . capacity () , 4) ; assert_eq ! (v . len () , 4) ; assert_eq ! (v [0] , 1) ; let v = const_new_inline_args () ; assert_eq ! (v . capacity () , 2) ; assert_eq ! (v . len () , 2) ; assert_eq ! (v [0] , 1) ; assert_eq ! (v [1] , 4) ; }
    };
}

const_new!()