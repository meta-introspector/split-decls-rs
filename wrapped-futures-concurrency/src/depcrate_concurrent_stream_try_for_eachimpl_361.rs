// Generated macro for impl_361 (impl)
macro_rules! Depcrate_concurrent_stream_try_for_eachimpl_361 {
() => {
// Module: crate::concurrent_stream::try_for_each
// Provides: {"impl_361"}
// Dependencies: {}
impl < F , FutT , T , FutB , B > Future for TryForEachFut < F , FutT , T , FutB , B > where FutT : Future < Output = T > , F : Clone + Fn (T) -> FutB , FutB : Future < Output = B > , B : Try < Output = () > , { type Output = B ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = unsafe { self . get_unchecked_mut () } ; if this . done { panic ! ("future has already been polled to completion once") ; } if let Some (fut) = this . fut_t . as_mut () { let t = ready ! (unsafe { Pin :: new_unchecked (fut) } . poll (cx)) ; let fut_b = (this . f) (t) ; this . fut_t = None ; this . fut_b = Some (fut_b) ; } if let Some (fut) = this . fut_b . as_mut () { let item = ready ! (unsafe { Pin :: new_unchecked (fut) } . poll (cx)) ; this . count . fetch_sub (1 , Ordering :: Relaxed) ; this . done = true ; return Poll :: Ready (item) ; } unreachable ! ("neither future `a` nor future `b` were ready") ; } }
};
}
