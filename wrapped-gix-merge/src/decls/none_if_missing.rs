macro_rules! none_if_missing {
    () => {
        fn none_if_missing < T > (res : std :: io :: Result < T >) -> std :: io :: Result < Option < T > > { match res { Ok (data) => Ok (Some (data)) , Err (err) if err . kind () == std :: io :: ErrorKind :: NotFound => Ok (None) , Err (err) => Err (err) , } }
    };
}

none_if_missing!();