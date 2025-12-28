macro_rules! deps {
    () => {
        RaceOk!();
        AggregateError!();
    };
}

macro_rules! impl_297 {
    () => {
        deps!();
        impl < Fut , T , E , const N : usize > Future for RaceOk < Fut , T , E , N > where Fut : Future < Output = Result < T , E > > , { type Output = Result < T , AggregateError < E , N > > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; let futures = iter_pin_mut (this . futures) ; for ((fut , out) , st) in futures . zip (this . errors . iter_mut ()) . zip (this . error_states . iter_mut ()) { if st . is_ready () { continue ; } if let Poll :: Ready (output) = fut . poll (cx) { match output { Ok (ok) => return Poll :: Ready (Ok (ok)) , Err (err) => { * out = MaybeUninit :: new (err) ; * this . completed += 1 ; st . set_ready () ; } } } } let all_completed = * this . completed == N ; if all_completed { let mut errors = array :: from_fn (| _ | MaybeUninit :: uninit ()) ; mem :: swap (& mut errors , this . errors) ; this . error_states . set_all_none () ; let result = unsafe { array_assume_init (errors) } ; Poll :: Ready (Err (AggregateError :: new (result))) } else { Poll :: Pending } } }
    };
}

impl_297!()