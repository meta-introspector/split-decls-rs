macro_rules! deps {
    () => {
        IntoIter!();
        ChunksExact!();
        Producer!();
        ChunksExactProducer!();
    };
}

macro_rules! impl_1156 {
    () => {
        deps!();
        impl < 'data , T : 'data + Sync > Producer for ChunksExactProducer < 'data , T > { type Item = & 'data [T] ; type IntoIter = :: std :: slice :: ChunksExact < 'data , T > ; fn into_iter (self) -> Self :: IntoIter { self . slice . chunks_exact (self . chunk_size) } fn split_at (self , index : usize) -> (Self , Self) { let elem_index = index * self . chunk_size ; let (left , right) = self . slice . split_at (elem_index) ; (ChunksExactProducer { chunk_size : self . chunk_size , slice : left , } , ChunksExactProducer { chunk_size : self . chunk_size , slice : right , } ,) } }
    };
}

impl_1156!()