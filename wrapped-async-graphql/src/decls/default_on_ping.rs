macro_rules! deps {
    () => {
        Data!();
        Result!();
    };
}

macro_rules! default_on_ping {
    () => {
        deps!();
        # [doc = " Default ping handler function."] pub fn default_on_ping (_ : Option < & Data > , _ : Option < serde_json :: Value > ,) -> Ready < Result < Option < serde_json :: Value > > > { futures_util :: future :: ready (Ok (None)) }
    };
}

default_on_ping!();