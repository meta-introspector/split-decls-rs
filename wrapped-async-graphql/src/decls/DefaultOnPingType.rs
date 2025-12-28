macro_rules! deps {
    () => {
        Result!();
        Data!();
    };
}

macro_rules! DefaultOnPingType {
    () => {
        deps!();
        # [doc = " Default ping handler type."] pub type DefaultOnPingType = fn (Option < & Data > , Option < serde_json :: Value >) -> Ready < Result < Option < serde_json :: Value > > > ;
    };
}

DefaultOnPingType!()