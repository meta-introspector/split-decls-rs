macro_rules! deps {
    () => {
        Compat01As03!();
    };
}

macro_rules! Future01CompatExt {
    () => {
        deps!();
        # [doc = " Extension trait for futures 0.1 [`Future`](futures_01::future::Future)"] pub trait Future01CompatExt : Future01 { # [doc = " Converts a futures 0.1"] # [doc = " [`Future<Item = T, Error = E>`](futures_01::future::Future)"] # [doc = " into a futures 0.3"] # [doc = " [`Future<Output = Result<T, E>>`](futures_core::future::Future)."] # [doc = ""] # [doc = " ```"] # [doc = " # if cfg!(miri) { return; } // https://github.com/rust-lang/futures-rs/issues/2514"] # [doc = " # futures::executor::block_on(async {"] # [doc = " # // TODO: These should be all using `futures::compat`, but that runs up against Cargo"] # [doc = " # // feature issues"] # [doc = " use futures_util::compat::Future01CompatExt;"] # [doc = ""] # [doc = " let future = futures_01::future::ok::<u32, ()>(1);"] # [doc = " assert_eq!(future.compat().await, Ok(1));"] # [doc = " # });"] # [doc = " ```"] fn compat (self) -> Compat01As03 < Self > where Self : Sized , { Compat01As03 :: new (self) } }
    };
}

Future01CompatExt!();