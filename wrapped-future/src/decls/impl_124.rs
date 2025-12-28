macro_rules! deps {
    () => {
        Waiter!();
        WaiterSignaler!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl Waiter { pub fn new () -> crate :: Result < (Self , WaiterSignaler) > { unsafe { let handle = CreateEventW (core :: ptr :: null () , 1 , 0 , core :: ptr :: null ()) ; if handle . is_null () { Err (crate :: Error :: from_thread ()) } else { Ok ((Self (handle) , WaiterSignaler (handle))) } } } }
    };
}

impl_124!()