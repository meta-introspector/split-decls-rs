macro_rules! Single {
    () => {
        # [doc = " Emits a single item immediately, then stream will be terminated."] # [derive (Debug , Clone)] pub struct Single < T > (Option < T >) ;
    };
}

Single!()