macro_rules! deps {
    () => {
        Timer!();
        Sleep!();
        Dur!();
        Time!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl Time { # [cfg (all (any (feature = "client" , feature = "server") , feature = "http2"))] pub (crate) fn sleep (& self , duration : Duration) -> Pin < Box < dyn Sleep > > { match * self { Time :: Empty => { panic ! ("You must supply a timer.") } Time :: Timer (ref t) => t . sleep (duration) , } } # [cfg (all (feature = "server" , feature = "http1"))] pub (crate) fn sleep_until (& self , deadline : Instant) -> Pin < Box < dyn Sleep > > { match * self { Time :: Empty => { panic ! ("You must supply a timer.") } Time :: Timer (ref t) => t . sleep_until (deadline) , } } pub (crate) fn now (& self) -> Instant { match * self { Time :: Empty => Instant :: now () , Time :: Timer (ref t) => t . now () , } } pub (crate) fn reset (& self , sleep : & mut Pin < Box < dyn Sleep > > , new_deadline : Instant) { match * self { Time :: Empty => { panic ! ("You must supply a timer.") } Time :: Timer (ref t) => t . reset (sleep , new_deadline) , } } # [cfg (all (feature = "server" , feature = "http1"))] pub (crate) fn check (& self , dur : Dur , name : & 'static str) -> Option < Duration > { match dur { Dur :: Default (Some (dur)) => match self { Time :: Empty => { warn ! ("timeout `{}` has default, but no timer set" , name ,) ; None } Time :: Timer (..) => Some (dur) , } , Dur :: Configured (Some (dur)) => match self { Time :: Empty => panic ! ("timeout `{}` set, but no timer set" , name ,) , Time :: Timer (..) => Some (dur) , } , Dur :: Default (None) | Dur :: Configured (None) => None , } } }
    };
}

impl_87!()