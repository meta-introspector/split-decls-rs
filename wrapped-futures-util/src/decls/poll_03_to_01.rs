macro_rules! deps {
    () => {
        Pending!();
        Ready!();
    };
}

macro_rules! poll_03_to_01 {
    () => {
        deps!();
        fn poll_03_to_01 < T , E > (x : task03 :: Poll < Result < T , E > >) -> Result < Async01 < T > , E > { match x ? { task03 :: Poll :: Ready (t) => Ok (Async01 :: Ready (t)) , task03 :: Poll :: Pending => Ok (Async01 :: NotReady) , } }
    };
}

poll_03_to_01!();