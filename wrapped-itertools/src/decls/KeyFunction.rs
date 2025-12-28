macro_rules! deps {
    () => {
        ChunkBy!();
        IntoChunks!();
    };
}

macro_rules! KeyFunction {
    () => {
        deps!();
        # [doc = " A trait to unify `FnMut` for `ChunkBy` with the chunk key in `IntoChunks`"] trait KeyFunction < A > { type Key ; fn call_mut (& mut self , arg : A) -> Self :: Key ; }
    };
}

KeyFunction!();