macro_rules! Delay {
    () => {
        # [doc = " A version of `Delay` that works on wasm."] # [derive (Debug)] pub struct Delay (SendWrapper < TimeoutFuture >) ;
    };
}

Delay!();