// Generated macro for impl_32 (impl)
macro_rules! Depcrateimpl_32 {
() => {
// Module: crate
// Provides: {"impl_32"}
// Dependencies: {}
impl core :: fmt :: Debug for jvalue { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { let b = unsafe { self . b } ; f . debug_struct ("jvalue") . field ("z" , & if b == 0 { "false" } else if b == 1 { "true" } else { "invalid" } ,) . field ("b" , unsafe { & self . b }) . field ("c" , unsafe { & self . c }) . field ("s" , unsafe { & self . s }) . field ("i" , unsafe { & self . i }) . field ("j" , unsafe { & self . j }) . field ("f" , unsafe { & self . f }) . field ("d" , unsafe { & self . d }) . field ("l" , unsafe { & self . l }) . finish () } }
};
}
