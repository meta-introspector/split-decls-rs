// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
impl Counter { pub fn new (callback : Global < JObject < 'static > >) -> Counter { Counter { count : 0 , callback : callback , } } pub fn increment (& mut self , env : & mut jni :: Env) { self . count = self . count + 1 ; env . call_method (& self . callback , c"counterCallback" , c"(I)V" , & [self . count . into ()] ,) . unwrap () ; } }
};
}
