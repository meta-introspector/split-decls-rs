// Generated macro for impl_54 (impl)
macro_rules! Depcrate_content_serializationimpl_54 {
() => {
// Module: crate::content::serialization
// Provides: {"impl_54"}
// Dependencies: {}
impl Ord for Key < '_ > { fn cmp (& self , other : & Self) -> Ordering { let self_discriminant = self . discriminant () ; let other_discriminant = other . discriminant () ; match Ord :: cmp (& self_discriminant , & other_discriminant) { Ordering :: Equal => match (self , other) { (Key :: Bool (a) , Key :: Bool (b)) => Ord :: cmp (a , b) , (Key :: U64 (a) , Key :: U64 (b)) => Ord :: cmp (a , b) , (Key :: I64 (a) , Key :: I64 (b)) => Ord :: cmp (a , b) , (Key :: F64 (a) , Key :: F64 (b)) => f64_total_cmp (* a , * b) , (Key :: U128 (a) , Key :: U128 (b)) => Ord :: cmp (a , b) , (Key :: I128 (a) , Key :: I128 (b)) => Ord :: cmp (a , b) , (Key :: Str (a) , Key :: Str (b)) => Ord :: cmp (a , b) , (Key :: Bytes (a) , Key :: Bytes (b)) => Ord :: cmp (a , b) , _ => Ordering :: Equal , } , cmp => cmp , } } }
};
}
