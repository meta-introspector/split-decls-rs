macro_rules! deps {
    () => {
        SenderFlavor!();
        Token!();
        Operation!();
        Sender!();
        Context!();
        SelectHandle!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < T > SelectHandle for Sender < T > { fn try_select (& self , token : & mut Token) -> bool { match & self . flavor { SenderFlavor :: Array (chan) => chan . sender () . try_select (token) , SenderFlavor :: List (chan) => chan . sender () . try_select (token) , SenderFlavor :: Zero (chan) => chan . sender () . try_select (token) , } } fn deadline (& self) -> Option < Instant > { None } fn register (& self , oper : Operation , cx : & Context) -> bool { match & self . flavor { SenderFlavor :: Array (chan) => chan . sender () . register (oper , cx) , SenderFlavor :: List (chan) => chan . sender () . register (oper , cx) , SenderFlavor :: Zero (chan) => chan . sender () . register (oper , cx) , } } fn unregister (& self , oper : Operation) { match & self . flavor { SenderFlavor :: Array (chan) => chan . sender () . unregister (oper) , SenderFlavor :: List (chan) => chan . sender () . unregister (oper) , SenderFlavor :: Zero (chan) => chan . sender () . unregister (oper) , } } fn accept (& self , token : & mut Token , cx : & Context) -> bool { match & self . flavor { SenderFlavor :: Array (chan) => chan . sender () . accept (token , cx) , SenderFlavor :: List (chan) => chan . sender () . accept (token , cx) , SenderFlavor :: Zero (chan) => chan . sender () . accept (token , cx) , } } fn is_ready (& self) -> bool { match & self . flavor { SenderFlavor :: Array (chan) => chan . sender () . is_ready () , SenderFlavor :: List (chan) => chan . sender () . is_ready () , SenderFlavor :: Zero (chan) => chan . sender () . is_ready () , } } fn watch (& self , oper : Operation , cx : & Context) -> bool { match & self . flavor { SenderFlavor :: Array (chan) => chan . sender () . watch (oper , cx) , SenderFlavor :: List (chan) => chan . sender () . watch (oper , cx) , SenderFlavor :: Zero (chan) => chan . sender () . watch (oper , cx) , } } fn unwatch (& self , oper : Operation) { match & self . flavor { SenderFlavor :: Array (chan) => chan . sender () . unwatch (oper) , SenderFlavor :: List (chan) => chan . sender () . unwatch (oper) , SenderFlavor :: Zero (chan) => chan . sender () . unwatch (oper) , } } }
    };
}

impl_40!()