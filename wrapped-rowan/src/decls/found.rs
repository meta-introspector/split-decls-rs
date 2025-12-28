macro_rules! found {
    () => {
        fn found < T > (res : Result < () , T >) -> Option < T > { match res { Ok (()) => None , Err (it) => Some (it) , } }
    };
}

found!()