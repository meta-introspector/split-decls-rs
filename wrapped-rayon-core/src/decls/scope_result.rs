macro_rules! scope_result {
    () => {
        # [test] fn scope_result () { let x = scope (| _ | 22) ; assert_eq ! (x , 22) ; }
    };
}

scope_result!()