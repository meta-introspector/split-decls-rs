// Generated macro for impl_18 (impl)
macro_rules! Depcrate_block_apiimpl_18 {
() => {
// Module: crate::block_api
// Provides: {"impl_18"}
// Dependencies: {}
impl < P : Gost94Params > Gost94Core < P > { fn shuffle (& mut self , m : & Block , s : & Block) { let mut res = Block :: default () ; res . copy_from_slice (s) ; for _ in 0 .. 12 { psi (& mut res) ; } x_mut (& mut res , m) ; psi (& mut res) ; x_mut (& mut self . h , & res) ; for _ in 0 .. 61 { psi (& mut self . h) ; } } fn f (& mut self , m : & Block) { let mut s = Block :: default () ; s . copy_from_slice (& self . h) ; let k = p (x (& self . h , m)) ; encrypt (& mut s [0 .. 8] , k , & P :: S_BOX) ; let u = a (self . h) ; let v = a (a (* m)) ; let k = p (x (& u , & v)) ; encrypt (& mut s [8 .. 16] , k , & P :: S_BOX) ; let mut u = a (u) ; x_mut (& mut u , & C) ; let v = a (a (v)) ; let k = p (x (& u , & v)) ; encrypt (& mut s [16 .. 24] , k , & P :: S_BOX) ; let u = a (u) ; let v = a (a (v)) ; let k = p (x (& u , & v)) ; encrypt (& mut s [24 .. 32] , k , & P :: S_BOX) ; self . shuffle (m , & s) ; } fn update_sigma (& mut self , m : & Block) { let mut carry = 0 ; for (a , chunk) in self . sigma . iter_mut () . zip (m . chunks_exact (8)) { let b = u64 :: from_le_bytes (chunk . try_into () . unwrap ()) ; adc (a , b , & mut carry) ; } } fn update_n (& mut self , len : usize) { let mut carry = 0 ; adc (& mut self . n [0] , (len as u64) << 3 , & mut carry) ; adc (& mut self . n [1] , (len as u64) >> 61 , & mut carry) ; adc (& mut self . n [2] , 0 , & mut carry) ; adc (& mut self . n [3] , 0 , & mut carry) ; } # [inline (always)] fn compress (& mut self , block : & [u8 ; 32]) { self . f (block) ; self . update_sigma (block) ; } }
};
}
