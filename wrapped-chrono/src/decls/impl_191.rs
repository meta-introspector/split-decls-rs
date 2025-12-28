macro_rules! deps {
    () => {
        Date!();
        Utc!();
        DateTime!();
    };
}

macro_rules! impl_191 {
    () => {
        deps!();
        # [cfg (all (target_arch = "wasm32" , feature = "wasmbind" , not (any (target_os = "emscripten" , target_os = "wasi" , target_os = "linux"))))] impl From < js_sys :: Date > for DateTime < Utc > { fn from (date : js_sys :: Date) -> DateTime < Utc > { DateTime :: < Utc > :: from (& date) } }
    };
}

impl_191!()