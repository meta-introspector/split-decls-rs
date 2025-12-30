// Generated macro for NumOrd (trait)
macro_rules! DepcrateNumOrd {
() => {
// Module: crate
// Provides: {"NumOrd"}
// Dependencies: {}
# [doc = " Consistent comparison among different numeric types."] pub trait NumOrd < Other > { # [doc = " [PartialOrd::partial_cmp] on different numeric types"] fn num_partial_cmp (& self , other : & Other) -> Option < Ordering > ; # [inline] # [doc = " [PartialEq::eq] on different numeric types"] fn num_eq (& self , other : & Other) -> bool { matches ! (self . num_partial_cmp (other) , Some (Ordering :: Equal)) } # [inline] # [doc = " [PartialEq::ne] on different numeric types"] fn num_ne (& self , other : & Other) -> bool { ! self . num_eq (other) } # [inline] # [doc = " [PartialOrd::lt] on different numeric types"] fn num_lt (& self , other : & Other) -> bool { matches ! (self . num_partial_cmp (other) , Some (Ordering :: Less)) } # [inline] # [doc = " [PartialOrd::le] on different numeric types"] fn num_le (& self , other : & Other) -> bool { matches ! (self . num_partial_cmp (other) , Some (Ordering :: Equal) | Some (Ordering :: Less)) } # [inline] # [doc = " [PartialOrd::gt] on different numeric types"] fn num_gt (& self , other : & Other) -> bool { matches ! (self . num_partial_cmp (other) , Some (Ordering :: Greater)) } # [inline] # [doc = " [PartialOrd::ge] on different numeric types"] fn num_ge (& self , other : & Other) -> bool { matches ! (self . num_partial_cmp (other) , Some (Ordering :: Equal) | Some (Ordering :: Greater)) } # [inline] # [doc = " [Ord::cmp] on different numeric types. It panics if either of the numeric values contains NaN."] fn num_cmp (& self , other : & Other) -> Ordering { self . num_partial_cmp (other) . unwrap () } }
};
}
