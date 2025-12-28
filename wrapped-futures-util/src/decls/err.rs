macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! err {
    () => {
        deps!();
        # [doc = " Create a future that is immediately ready with an error value."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # futures::executor::block_on(async {"] # [doc = " use futures::future;"] # [doc = ""] # [doc = " let a = future::err::<i32, i32>(1);"] # [doc = " assert_eq!(a.await, Err(1));"] # [doc = " # });"] # [doc = " ```"] pub fn err < T , E > (err : E) -> Ready < Result < T , E > > { Ready (Some (Err (err))) }
    };
}

err!();