// Generated macro for try_iter (function)
macro_rules! Depcratetry_iter {
() => {
// Module: crate
// Provides: {"try_iter"}
// Dependencies: {}
# [doc = " Create an iterator over `val` using the JS iteration protocol and"] # [doc = " `Symbol.iterator`."] pub fn try_iter (val : & JsValue) -> Result < Option < IntoIter > , JsValue > { let iter_sym = Symbol :: iterator () ; let iter_fn = Reflect :: get (val , iter_sym . as_ref ()) ? ; let iter_fn : Function = match iter_fn . dyn_into () { Ok (iter_fn) => iter_fn , Err (_) => return Ok (None) , } ; let it : Iterator = match iter_fn . call0 (val) ? . dyn_into () { Ok (it) => it , Err (_) => return Ok (None) , } ; Ok (Some (it . into_iter ())) }
};
}
