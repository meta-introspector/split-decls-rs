macro_rules! ChunkByProducer {
    () => {
        struct ChunkByProducer < 'p , T , Slice , Pred > { slice : Slice , pred : & 'p Pred , tail : usize , marker : PhantomData < fn (& T) > , }
    };
}

ChunkByProducer!();