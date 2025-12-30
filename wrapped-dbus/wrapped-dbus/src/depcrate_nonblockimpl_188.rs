// Generated macro for impl_188 (impl)
macro_rules! Depcrate_nonblockimpl_188 {
() => {
// Module: crate::nonblock
// Provides: {"impl_188"}
// Dependencies: {}
impl < 'a , T , C > Proxy < 'a , C > where T : NonblockReply , C : std :: ops :: Deref < Target = T > { fn method_call_setup (& self , msg : Message) -> MRAwait { let mr = Arc :: new (Mutex :: new (MRInner :: Neither)) ; let mrouter = MROuter (mr . clone ()) ; let f = T :: make_f (move | msg : Message , _ : & T | { let mut inner = mr . lock () . unwrap () ; let old = mem :: replace (& mut * inner , MRInner :: Ready (Ok (msg))) ; if let MRInner :: Pending (waker) = old { waker . wake () } }) ; let timeout = Instant :: now () + self . timeout ; let token = self . connection . send_with_reply (msg , f) ; let timeoutfn = self . connection . timeout_maker () ; MRAwait { mrouter , token , timeout , timeoutfn } } # [doc = " Make a method call using typed input argument, returns a future that resolves to the typed output arguments."] pub fn method_call < 'i , 'm , R : ReadAll + 'static , A : AppendAll , I : Into < Interface < 'i > > , M : Into < Member < 'm > > > (& self , i : I , m : M , args : A) -> MethodReply < R > { let mut msg = Message :: method_call (& self . destination , & self . path , & i . into () , & m . into ()) ; args . append (& mut IterAppend :: new (& mut msg)) ; let mra = self . method_call_setup (msg) ; let r = method_call_await (mra) ; let r = futures_util :: FutureExt :: map (r , | r | -> Result < R , Error > { r . and_then (| rmsg | rmsg . read_all ()) }) ; MethodReply :: new (r) } }
};
}
