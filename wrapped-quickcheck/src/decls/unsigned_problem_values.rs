macro_rules! unsigned_problem_values {
    () => {
        macro_rules ! unsigned_problem_values { ($ t : ty) => { & [<$ t >:: MIN , 1 , <$ t >:: MAX] } ; }
    };
}

unsigned_problem_values!();