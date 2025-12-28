macro_rules! deps {
    () => {
        Result!();
        Data!();
    };
}

macro_rules! default_on_connection_init {
    () => {
        deps!();
        # [doc = " Default connection initializer function."] pub fn default_on_connection_init (_ : serde_json :: Value) -> Ready < Result < Data > > { futures_util :: future :: ready (Ok (Data :: default ())) }
    };
}

default_on_connection_init!()