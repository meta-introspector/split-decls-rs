macro_rules! needs_exact_match {
    () => {
        fn needs_exact_match (percentage : Option < f32 >) -> bool { percentage . is_none_or (| p | p >= 1.0) }
    };
}

needs_exact_match!()