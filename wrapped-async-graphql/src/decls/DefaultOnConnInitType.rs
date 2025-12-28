macro_rules! deps {
    () => {
        Result!();
        Data!();
    };
}

macro_rules! DefaultOnConnInitType {
    () => {
        deps!();
        # [doc = " Default connection initializer type."] pub type DefaultOnConnInitType = fn (serde_json :: Value) -> Ready < Result < Data > > ;
    };
}

DefaultOnConnInitType!()