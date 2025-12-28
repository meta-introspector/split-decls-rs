macro_rules! deps {
    () => {
        Data!();
        Result!();
    };
}

macro_rules! DefaultOnConnInitType {
    () => {
        deps!();
        # [doc = " Default connection initializer type."] pub type DefaultOnConnInitType = fn (serde_json :: Value) -> Ready < Result < Data > > ;
    };
}

DefaultOnConnInitType!();