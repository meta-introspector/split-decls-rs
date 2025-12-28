macro_rules! deps {
    () => {
        ChunkBySlice!();
    };
}

macro_rules! impl_1129 {
    () => {
        deps!();
        impl < T : Sync > ChunkBySlice < T > for & [T] { fn split (self , index : usize) -> (Self , Self) { self . split_at (index) } fn chunk_by (self , pred : & impl Fn (& T , & T) -> bool) -> impl Iterator < Item = Self > { self . chunk_by (pred) } }
    };
}

impl_1129!();