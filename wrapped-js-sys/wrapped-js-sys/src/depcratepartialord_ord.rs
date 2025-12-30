// Generated macro for partialord_ord (macro)
macro_rules! Depcratepartialord_ord {
() => {
// Module: crate
// Provides: {"partialord_ord"}
// Dependencies: {}
macro_rules ! partialord_ord { ($ t : ident) => { impl PartialOrd for $ t { # [inline] fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } # [inline] fn lt (& self , other : & Self) -> bool { JsValue :: as_ref (self) . lt (JsValue :: as_ref (other)) } # [inline] fn le (& self , other : & Self) -> bool { JsValue :: as_ref (self) . le (JsValue :: as_ref (other)) } # [inline] fn ge (& self , other : & Self) -> bool { JsValue :: as_ref (self) . ge (JsValue :: as_ref (other)) } # [inline] fn gt (& self , other : & Self) -> bool { JsValue :: as_ref (self) . gt (JsValue :: as_ref (other)) } } impl Ord for $ t { # [inline] fn cmp (& self , other : & Self) -> Ordering { if self == other { Ordering :: Equal } else if self . lt (other) { Ordering :: Less } else { Ordering :: Greater } } } } ; }
};
}
