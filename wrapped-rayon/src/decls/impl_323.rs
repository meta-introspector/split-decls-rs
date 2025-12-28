macro_rules! deps {
    () => {
        IntoIter!();
        ChunkSeq!();
        Producer!();
    };
}

macro_rules! impl_323 {
    () => {
        deps!();
        impl < P > Iterator for ChunkSeq < P > where P : Producer , { type Item = P :: IntoIter ; fn next (& mut self) -> Option < Self :: Item > { let producer = self . inner . take () ? ; if self . len > self . chunk_size { let (left , right) = producer . split_at (self . chunk_size) ; self . inner = Some (right) ; self . len -= self . chunk_size ; Some (left . into_iter ()) } else { debug_assert ! (self . len > 0) ; self . len = 0 ; Some (producer . into_iter ()) } } fn size_hint (& self) -> (usize , Option < usize >) { let len = self . len () ; (len , Some (len)) } }
    };
}

impl_323!()