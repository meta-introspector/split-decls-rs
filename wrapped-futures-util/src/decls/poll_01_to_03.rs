macro_rules! deps {
    () => {
        Ready!();
        Pending!();
    };
}

macro_rules! poll_01_to_03 {
    () => {
        deps!();
        fn poll_01_to_03 < T , E > (x : Result < Async01 < T > , E >) -> task03 :: Poll < Result < T , E > > { match x ? { Async01 :: Ready (t) => task03 :: Poll :: Ready (Ok (t)) , Async01 :: NotReady => task03 :: Poll :: Pending , } }
    };
}

poll_01_to_03!()