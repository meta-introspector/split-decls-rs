macro_rules! deps {
    () => {
        Producer!();
        ChunksExactMutProducer!();
        ChunksExactMut!();
        IntoIter!();
    };
}

macro_rules! impl_1168 {
    () => {
        deps!();
        impl < 'data , T : 'data + Send > Producer for ChunksExactMutProducer < 'data , T > { type Item = & 'data mut [T] ; type IntoIter = :: std :: slice :: ChunksExactMut < 'data , T > ; fn into_iter (self) -> Self :: IntoIter { self . slice . chunks_exact_mut (self . chunk_size) } fn split_at (self , index : usize) -> (Self , Self) { let elem_index = index * self . chunk_size ; let (left , right) = self . slice . split_at_mut (elem_index) ; (ChunksExactMutProducer { chunk_size : self . chunk_size , slice : left , } , ChunksExactMutProducer { chunk_size : self . chunk_size , slice : right , } ,) } }
    };
}

impl_1168!();