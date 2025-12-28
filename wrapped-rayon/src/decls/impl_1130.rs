macro_rules! deps {
    () => {
        ChunkBySlice!();
    };
}

macro_rules! impl_1130 {
    () => {
        deps!();
        impl < T : Send > ChunkBySlice < T > for & mut [T] { fn split (self , index : usize) -> (Self , Self) { self . split_at_mut (index) } fn chunk_by (self , pred : & impl Fn (& T , & T) -> bool) -> impl Iterator < Item = Self > { self . chunk_by_mut (pred) } }
    };
}

impl_1130!()