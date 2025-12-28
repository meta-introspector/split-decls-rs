macro_rules! deps {
    () => {
        DateTime!();
        Date!();
        Utc!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        # [cfg (all (target_arch = "wasm32" , feature = "wasmbind" , not (any (target_os = "emscripten" , target_os = "wasi" , target_os = "linux"))))] impl From < DateTime < Utc > > for js_sys :: Date { # [doc = " Converts a `DateTime<Utc>` to a JS `Date`. The resulting value may be lossy,"] # [doc = " any values that have a millisecond timestamp value greater/less than ±8,640,000,000,000,000"] # [doc = " (April 20, 271821 BCE ~ September 13, 275760 CE) will become invalid dates in JS."] fn from (date : DateTime < Utc >) -> js_sys :: Date { let js_millis = wasm_bindgen :: JsValue :: from_f64 (date . timestamp_millis () as f64) ; js_sys :: Date :: new (& js_millis) } }
    };
}

impl_193!()