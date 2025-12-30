// Generated macro for impl_43 (impl)
macro_rules! Depcrateimpl_43 {
() => {
// Module: crate
// Provides: {"impl_43"}
// Dependencies: {}
impl BigInt { # [doc = " Creates a new BigInt value."] # [doc = ""] # [doc = " [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt/BigInt)"] # [inline] pub fn new (value : & JsValue) -> Result < BigInt , Error > { new_bigint (value) } # [doc = " Applies the binary `/` JS operator on two `BigInt`s, catching and returning any `RangeError` thrown."] # [doc = ""] # [doc = " [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Division)"] pub fn checked_div (& self , rhs : & Self) -> Result < Self , RangeError > { let result = JsValue :: as_ref (self) . checked_div (JsValue :: as_ref (rhs)) ; if result . is_instance_of :: < RangeError > () { Err (result . unchecked_into ()) } else { Ok (result . unchecked_into ()) } } # [doc = " Applies the binary `**` JS operator on the two `BigInt`s."] # [doc = ""] # [doc = " [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Exponentiation)"] # [inline] pub fn pow (& self , rhs : & Self) -> Self { JsValue :: as_ref (self) . pow (JsValue :: as_ref (rhs)) . unchecked_into () } # [doc = " Returns a tuple of this [`BigInt`]'s absolute value along with a"] # [doc = " [`bool`] indicating whether the [`BigInt`] was negative."] fn abs (& self) -> (Self , bool) { if self < & BigInt :: from (0) { (- self , true) } else { (self . clone () , false) } } }
};
}
