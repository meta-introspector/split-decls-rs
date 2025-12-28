macro_rules! deps {
    () => {
        FnOnce1!();
        Ready!();
    };
}

macro_rules! impl_416 {
    () => {
        deps!();
        impl < St , F > Future for NextIf < '_ , St , F > where St : Stream , F : for < 'a > FnOnce1 < & 'a St :: Item , Output = bool > , { type Output = Option < St :: Item > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let inner = self . project () . inner ; if let Some ((peekable , _)) = inner { let res = ready ! (peekable . as_mut () . poll_next (cx)) ; let (peekable , func) = inner . take () . unwrap () ; match res { Some (ref matched) if func . call_once (matched) => Poll :: Ready (res) , other => { let peekable = peekable . project () ; assert ! (peekable . peeked . is_none ()) ; * peekable . peeked = other ; Poll :: Ready (None) } } } else { panic ! ("NextIf polled after completion") } } }
    };
}

impl_416!()