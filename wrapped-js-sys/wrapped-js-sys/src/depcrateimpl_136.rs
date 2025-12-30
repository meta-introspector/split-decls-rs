// Generated macro for impl_136 (impl)
macro_rules! Depcrateimpl_136 {
() => {
// Module: crate
// Provides: {"impl_136"}
// Dependencies: {}
impl PartialOrd for Number { # [inline] fn partial_cmp (& self , other : & Self) -> Option < Ordering > { if Number :: is_nan (self) || Number :: is_nan (other) { None } else if self == other { Some (Ordering :: Equal) } else if self . lt (other) { Some (Ordering :: Less) } else { Some (Ordering :: Greater) } } # [inline] fn lt (& self , other : & Self) -> bool { JsValue :: as_ref (self) . lt (JsValue :: as_ref (other)) } # [inline] fn le (& self , other : & Self) -> bool { JsValue :: as_ref (self) . le (JsValue :: as_ref (other)) } # [inline] fn ge (& self , other : & Self) -> bool { JsValue :: as_ref (self) . ge (JsValue :: as_ref (other)) } # [inline] fn gt (& self , other : & Self) -> bool { JsValue :: as_ref (self) . gt (JsValue :: as_ref (other)) } }
};
}
