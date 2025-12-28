macro_rules! VecConsumer {
    () => {
        # [pin_project] pub (crate) struct VecConsumer < 'a , Fut : Future > { # [pin] group : FuturesUnordered < Fut > , output : & 'a mut Vec < Fut :: Output > , }
    };
}

VecConsumer!();