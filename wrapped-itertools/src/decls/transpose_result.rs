macro_rules! transpose_result {
    () => {
        fn transpose_result < T , E > (result : Result < Option < T > , E >) -> Option < Result < T , E > > { match result { Ok (Some (v)) => Some (Ok (v)) , Ok (None) => None , Err (e) => Some (Err (e)) , } }
    };
}

transpose_result!()