macro_rules! deps {
    () => {
        KeyFunction!();
        ChunkIndex!();
    };
}

macro_rules! impl_255 {
    () => {
        deps!();
        impl < A > KeyFunction < A > for ChunkIndex { type Key = usize ; # [inline (always)] fn call_mut (& mut self , _arg : A) -> Self :: Key { if self . index == self . size { self . key += 1 ; self . index = 0 ; } self . index += 1 ; self . key } }
    };
}

impl_255!()