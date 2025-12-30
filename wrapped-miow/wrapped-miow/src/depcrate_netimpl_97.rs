// Generated macro for impl_97 (impl)
macro_rules! Depcrate_netimpl_97 {
() => {
// Module: crate::net
// Provides: {"impl_97"}
// Dependencies: {}
impl WsaExtension { fn get (& self , socket : SOCKET) -> io :: Result < usize > { let prev = self . val . load (Ordering :: SeqCst) ; if prev != 0 && ! cfg ! (debug_assertions) { return Ok (prev) ; } let mut ret = 0_usize ; let mut bytes = 0 ; const SIO_GET_EXTENSION_FUNCTION_POINTER : u32 = 33_5544_3206u32 ; let r = unsafe { WSAIoctl (socket , SIO_GET_EXTENSION_FUNCTION_POINTER , & self . guid as * const _ as * mut _ , mem :: size_of_val (& self . guid) as u32 , & mut ret as * mut _ as * mut _ , mem :: size_of_val (& ret) as u32 , & mut bytes , std :: ptr :: null_mut () , None ,) } ; cvt (r , 0) . map (| _ | { debug_assert_eq ! (bytes as usize , mem :: size_of_val (& ret)) ; debug_assert ! (prev == 0 || prev == ret) ; self . val . store (ret , Ordering :: SeqCst) ; ret }) } }
};
}
