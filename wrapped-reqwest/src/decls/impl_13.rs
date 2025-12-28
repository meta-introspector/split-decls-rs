macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        # [cfg (target_arch = "wasm32")] impl From < crate :: error :: Error > for js_sys :: Error { fn from (err : Error) -> js_sys :: Error { js_sys :: Error :: new (& format ! ("{err}")) } }
    };
}

impl_13!();