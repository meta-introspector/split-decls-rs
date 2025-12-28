macro_rules! deps {
    () => {
        Lzma2Options!();
        WorkPool!();
        WorkUnit!();
        Write!();
    };
}

macro_rules! Lzma2WriterMt {
    () => {
        deps!();
        # [doc = " A multi-threaded LZMA2 compressor."] pub struct Lzma2WriterMt < W : Write > { inner : W , options : Lzma2Options , chunk_size : usize , current_work_unit : Vec < u8 > , work_pool : WorkPool < WorkUnit , Vec < u8 > > , }
    };
}

Lzma2WriterMt!()