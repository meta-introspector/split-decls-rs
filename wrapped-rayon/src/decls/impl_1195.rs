macro_rules! deps {
    () => {
        IntoIter!();
        RChunksExactMut!();
        Producer!();
        RChunksExactMutProducer!();
    };
}

macro_rules! impl_1195 {
    () => {
        deps!();
        impl < 'data , T : 'data + Send > Producer for RChunksExactMutProducer < 'data , T > { type Item = & 'data mut [T] ; type IntoIter = :: std :: slice :: RChunksExactMut < 'data , T > ; fn into_iter (self) -> Self :: IntoIter { self . slice . rchunks_exact_mut (self . chunk_size) } fn split_at (self , index : usize) -> (Self , Self) { let elem_index = self . slice . len () - index * self . chunk_size ; let (left , right) = self . slice . split_at_mut (elem_index) ; (RChunksExactMutProducer { chunk_size : self . chunk_size , slice : right , } , RChunksExactMutProducer { chunk_size : self . chunk_size , slice : left , } ,) } }
    };
}

impl_1195!();