macro_rules! deps {
    () => {
        MapFuture!();
    };
}

macro_rules! impl_176 {
    () => {
        deps!();
        impl < F , FutT , T , FutB , B > Future for MapFuture < F , FutT , T , FutB , B > where FutT : Future < Output = T > , F : Fn (T) -> FutB , FutB : Future < Output = B > , { type Output = B ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = unsafe { self . get_unchecked_mut () } ; if this . done { panic ! ("future has already been polled to completion once") ; } if let Some (fut) = this . fut_t . as_mut () { let t = ready ! (unsafe { Pin :: new_unchecked (fut) } . poll (cx)) ; let fut_b = (this . f) (t) ; this . fut_t = None ; this . fut_b = Some (fut_b) ; } if let Some (fut) = this . fut_b . as_mut () { let t = ready ! (unsafe { Pin :: new_unchecked (fut) } . poll (cx)) ; this . done = true ; return Poll :: Ready (t) ; } unreachable ! ("neither future `a` nor future `b` were ready") ; } }
    };
}

impl_176!()