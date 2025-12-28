macro_rules! deps {
    () => {
        Utc!();
        DateTime!();
        Date!();
    };
}

macro_rules! impl_192 {
    () => {
        deps!();
        # [cfg (all (target_arch = "wasm32" , feature = "wasmbind" , not (any (target_os = "emscripten" , target_os = "wasi" , target_os = "linux"))))] impl From < & js_sys :: Date > for DateTime < Utc > { fn from (date : & js_sys :: Date) -> DateTime < Utc > { Utc . timestamp_millis_opt (date . get_time () as i64) . unwrap () } }
    };
}

impl_192!();