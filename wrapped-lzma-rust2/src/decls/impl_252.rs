macro_rules! deps {
    () => {
        Read!();
        Lzma2ReaderMt!();
    };
}

macro_rules! impl_252 {
    () => {
        deps!();
        impl < R : Read > Drop for Lzma2ReaderMt < R > { fn drop (& mut self) { self . shutdown_flag . store (true , Ordering :: Release) ; self . work_queue . close () ; } }
    };
}

impl_252!()