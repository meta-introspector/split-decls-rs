macro_rules! float_problem_values {
    () => {
        macro_rules ! float_problem_values { ($ t : ty) => { { & [<$ t >:: NAN , <$ t >:: NEG_INFINITY , <$ t >:: MIN , - 0. , 0. , <$ t >:: MAX , <$ t >:: INFINITY ,] } } ; }
    };
}

float_problem_values!()