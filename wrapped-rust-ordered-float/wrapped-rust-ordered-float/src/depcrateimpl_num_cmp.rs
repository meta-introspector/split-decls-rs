// Generated macro for impl_num_cmp (module)
macro_rules! Depcrateimpl_num_cmp {
() => {
// Module: crate
// Provides: {"impl_num_cmp"}
// Dependencies: {}
# [cfg (feature = "num-cmp")] mod impl_num_cmp { use super :: OrderedFloat ; use core :: cmp :: Ordering ; use num_cmp :: NumCmp ; use num_traits :: float :: FloatCore ; impl < T , U > NumCmp < U > for OrderedFloat < T > where T : FloatCore + NumCmp < U > , U : Copy , { fn num_cmp (self , other : U) -> Option < Ordering > { NumCmp :: num_cmp (self . 0 , other) } fn num_eq (self , other : U) -> bool { NumCmp :: num_eq (self . 0 , other) } fn num_ne (self , other : U) -> bool { NumCmp :: num_ne (self . 0 , other) } fn num_lt (self , other : U) -> bool { NumCmp :: num_lt (self . 0 , other) } fn num_gt (self , other : U) -> bool { NumCmp :: num_gt (self . 0 , other) } fn num_le (self , other : U) -> bool { NumCmp :: num_le (self . 0 , other) } fn num_ge (self , other : U) -> bool { NumCmp :: num_ge (self . 0 , other) } } # [test] pub fn test_num_cmp () { let f = OrderedFloat (1.0) ; assert_eq ! (NumCmp :: num_cmp (f , 1.0) , Some (Ordering :: Equal)) ; assert_eq ! (NumCmp :: num_cmp (f , - 1.0) , Some (Ordering :: Greater)) ; assert_eq ! (NumCmp :: num_cmp (f , 2.0) , Some (Ordering :: Less)) ; assert ! (NumCmp :: num_eq (f , 1)) ; assert ! (NumCmp :: num_ne (f , - 1)) ; assert ! (NumCmp :: num_lt (f , 100)) ; assert ! (NumCmp :: num_gt (f , 0)) ; assert ! (NumCmp :: num_le (f , 1)) ; assert ! (NumCmp :: num_le (f , 2)) ; assert ! (NumCmp :: num_ge (f , 1)) ; assert ! (NumCmp :: num_ge (f , - 1)) ; } }
};
}
