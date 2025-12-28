macro_rules! deps {
    () => {
        Buffers!();
    };
}

macro_rules! WithForeignSource {
    () => {
        deps!();
        # [doc = " A utility to do buffer-swapping with, similar to [`Buffers`], but with support for a"] # [doc = " read-only one-time buffer as source."] pub struct WithForeignSource < 'src , 'bufs > { # [doc = " The original source buffer, or `None` if already altered."] pub ro_src : Option < & 'src [u8] > , # [doc = " The source buffer that will be used after the first call to `swap`."] pub src : & 'bufs mut Vec < u8 > , dest : & 'bufs mut Vec < u8 > , }
    };
}

WithForeignSource!();