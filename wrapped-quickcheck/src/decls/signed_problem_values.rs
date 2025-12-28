macro_rules! signed_problem_values {
    () => {
        macro_rules ! signed_problem_values { ($ t : ty) => { & [<$ t >:: MIN , 0 , <$ t >:: MAX] } ; }
    };
}

signed_problem_values!();