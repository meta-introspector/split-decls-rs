macro_rules! deps {
    () => {
        Pending!();
        Ready!();
        TryJoinAll!();
        TryJoinAllKind!();
        FinalState!();
    };
}

macro_rules! impl_249 {
    () => {
        deps!();
        impl < F > Future for TryJoinAll < F > where F : TryFuture , { type Output = Result < Vec < F :: Ok > , F :: Error > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { match & mut self . kind { TryJoinAllKind :: Small { elems } => { let mut state = FinalState :: AllDone ; for elem in join_all :: iter_pin_mut (elems . as_mut ()) { match elem . try_poll (cx) { Poll :: Pending => state = FinalState :: Pending , Poll :: Ready (Ok (())) => { } Poll :: Ready (Err (e)) => { state = FinalState :: Error (e) ; break ; } } } match state { FinalState :: Pending => Poll :: Pending , FinalState :: AllDone => { let mut elems = mem :: replace (elems , Box :: pin ([])) ; let results = join_all :: iter_pin_mut (elems . as_mut ()) . map (| e | e . take_output () . unwrap ()) . collect () ; Poll :: Ready (Ok (results)) } FinalState :: Error (e) => { let _ = mem :: replace (elems , Box :: pin ([])) ; Poll :: Ready (Err (e)) } } } # [cfg_attr (target_os = "none" , cfg (target_has_atomic = "ptr"))] TryJoinAllKind :: Big { fut } => Pin :: new (fut) . poll (cx) , } } }
    };
}

impl_249!()