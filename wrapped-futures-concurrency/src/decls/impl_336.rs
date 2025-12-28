macro_rules! deps {
    () => {
        RaceOk!();
        AggregateError!();
    };
}

macro_rules! impl_336 {
    () => {
        deps!();
        impl < Fut , T , E > Future for RaceOk < Fut , T , E > where Fut : Future < Output = Result < T , E > > , { type Output = Result < T , AggregateError < E > > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut all_done = true ; for mut elem in iter_pin_mut (self . elems . as_mut ()) { if elem . as_mut () . poll (cx) . is_pending () { all_done = false } else if let Some (output) = elem . take_ok () { return Poll :: Ready (Ok (output)) ; } } if all_done { let mut elems = mem :: replace (& mut self . elems , Box :: pin ([])) ; let result : Vec < E > = iter_pin_mut (elems . as_mut ()) . map (| e | match e . take_err () { Some (err) => err , None => unreachable ! () , }) . collect () ; Poll :: Ready (Err (AggregateError :: new (result))) } else { Poll :: Pending } } }
    };
}

impl_336!()