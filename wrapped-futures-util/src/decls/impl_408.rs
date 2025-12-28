macro_rules! impl_408 {
    () => {
        impl < 'a , St > Future for Peek < 'a , St > where St : Stream , { type Output = Option < & 'a St :: Item > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let inner = self . project () . inner ; if let Some (peekable) = inner { ready ! (peekable . as_mut () . poll_peek (cx)) ; inner . take () . unwrap () . poll_peek (cx) } else { panic ! ("Peek polled after completion") } } }
    };
}

impl_408!();