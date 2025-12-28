macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! is_private {
    () => {
        deps!();
        # [cfg (feature = "serde")] fn is_private (data : & Data) -> bool { match data { Data :: Private => true , Data :: Struct (_) | Data :: Enum (_) => false , } }
    };
}

is_private!()