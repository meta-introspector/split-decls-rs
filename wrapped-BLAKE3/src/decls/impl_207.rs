macro_rules! deps {
    () => {
        Hasher!();
    };
}

macro_rules! impl_207 {
    () => {
        deps!();
        # [cfg (feature = "zeroize")] impl Zeroize for Hasher { fn zeroize (& mut self) { let Self { key , chunk_state , initial_chunk_counter , cv_stack , } = self ; key . zeroize () ; chunk_state . zeroize () ; initial_chunk_counter . zeroize () ; cv_stack . zeroize () ; } }
    };
}

impl_207!()