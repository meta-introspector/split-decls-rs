macro_rules! IndentHeuristic {
    () => {
        pub struct IndentHeuristic < IndentOfToken > { indent_of_token : IndentOfToken , }
    };
}

IndentHeuristic!()