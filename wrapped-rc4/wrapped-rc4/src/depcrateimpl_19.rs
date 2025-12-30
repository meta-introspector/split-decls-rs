// Generated macro for impl_19 (impl)
macro_rules! Depcrateimpl_19 {
() => {
// Module: crate
// Provides: {"impl_19"}
// Dependencies: {}
impl Rc4State { fn new (key : & [u8]) -> Self { let mut state = Self { state : [0 ; 256] , i : 0 , j : 0 , } ; state . ksa (key) ; state } fn ksa (& mut self , key : & [u8]) { self . state . iter_mut () . enumerate () . for_each (| (i , x) | { * x = i as u8 ; }) ; let i_iter = 0 .. 256usize ; let key_iter = key . iter () . cycle () ; let mut j = 0u8 ; i_iter . zip (key_iter) . for_each (| (i , k) | { j = j . wrapping_add (self . state [i]) . wrapping_add (* k) ; self . state . swap (i , j . into ()) ; }) ; } fn s_i (& self) -> u8 { self . state [self . i as usize] } fn s_j (& self) -> u8 { self . state [self . j as usize] } fn prga (& mut self) -> u8 { self . i = self . i . wrapping_add (1) ; self . j = self . j . wrapping_add (self . s_i ()) ; self . state . swap (self . i . into () , self . j . into ()) ; let index : usize = self . s_i () . wrapping_add (self . s_j ()) . into () ; self . state [index] } }
};
}
