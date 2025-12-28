macro_rules! deps {
    () => {
        Thread!();
    };
}

macro_rules! spawn_thread {
    () => {
        deps!();
        fn spawn_thread (f : Box < dyn FnOnce () > , stack_size : Option < usize >) -> Thread { let body = move | | { loop { let f : Option < Option < Box < dyn FnOnce () > > > = generator :: yield_ (()) ; if let Some (f) = f { generator :: yield_with (()) ; f . unwrap () () ; } else { break ; } } generator :: done ! () ; } ; let mut g = match stack_size { Some (stack_size) => Gn :: new_opt (stack_size , body) , None => Gn :: new (body) , } ; g . resume () ; g . set_para (Some (f)) ; g }
    };
}

spawn_thread!();