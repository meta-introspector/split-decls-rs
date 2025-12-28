macro_rules! ResultVecConsumer {
    () => {
        # [pin_project] pub (crate) struct ResultVecConsumer < 'a , Fut : Future , T , E > { # [pin] group : FuturesUnordered < Fut > , output : & 'a mut Result < Vec < T > , E > , }
    };
}

ResultVecConsumer!()