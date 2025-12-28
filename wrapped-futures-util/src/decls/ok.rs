macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! ok {
    () => {
        deps!();
        # [doc = " Create a future that is immediately ready with a success value."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # futures::executor::block_on(async {"] # [doc = " use futures::future;"] # [doc = ""] # [doc = " let a = future::ok::<i32, i32>(1);"] # [doc = " assert_eq!(a.await, Ok(1));"] # [doc = " # });"] # [doc = " ```"] pub fn ok < T , E > (t : T) -> Ready < Result < T , E > > { Ready (Some (Ok (t))) }
    };
}

ok!()