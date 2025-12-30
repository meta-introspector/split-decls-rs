// Generated macro for impl_8 (impl)
macro_rules! Depcrateimpl_8 {
() => {
// Module: crate
// Provides: {"impl_8"}
// Dependencies: {}
impl < T > Vec2 < T > { # [inline] fn new (value : T , len : [usize ; 2]) -> Self where T : Clone , { Vec2 { len , data : vec ! [value ; len [0] * len [1]] , } } # [inline] fn get (& self , index : [usize ; 2]) -> & T { debug_assert ! (index [0] < self . len [0]) ; debug_assert ! (index [1] < self . len [1]) ; & self . data [index [0] * self . len [1] + index [1]] } # [inline] fn set (& mut self , index : [usize ; 2] , value : T) { debug_assert ! (index [0] < self . len [0]) ; debug_assert ! (index [1] < self . len [1]) ; self . data [index [0] * self . len [1] + index [1]] = value ; } }
};
}
