macro_rules! deps {
    () => {
        Producer!();
        RChunksExactProducer!();
        IntoIter!();
        RChunksExact!();
    };
}

macro_rules! impl_1183 {
    () => {
        deps!();
        impl < 'data , T : 'data + Sync > Producer for RChunksExactProducer < 'data , T > { type Item = & 'data [T] ; type IntoIter = :: std :: slice :: RChunksExact < 'data , T > ; fn into_iter (self) -> Self :: IntoIter { self . slice . rchunks_exact (self . chunk_size) } fn split_at (self , index : usize) -> (Self , Self) { let elem_index = self . slice . len () - index * self . chunk_size ; let (left , right) = self . slice . split_at (elem_index) ; (RChunksExactProducer { chunk_size : self . chunk_size , slice : right , } , RChunksExactProducer { chunk_size : self . chunk_size , slice : left , } ,) } }
    };
}

impl_1183!()