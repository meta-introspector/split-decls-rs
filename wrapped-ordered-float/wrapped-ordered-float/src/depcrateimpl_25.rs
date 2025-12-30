// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
impl < T : FloatCore > PartialOrd for OrderedFloat < T > { # [inline] fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } # [inline] fn lt (& self , other : & Self) -> bool { ! self . ge (other) } # [inline] fn le (& self , other : & Self) -> bool { other . ge (self) } # [inline] fn gt (& self , other : & Self) -> bool { ! other . ge (self) } # [inline] fn ge (& self , other : & Self) -> bool { self . 0 . is_nan () | (self . 0 >= other . 0) } }
};
}
