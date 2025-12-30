// Generated macro for CaptureNames (struct)
macro_rules! Depcrate_re_bytesCaptureNames {
() => {
// Module: crate::re_bytes
// Provides: {"CaptureNames"}
// Dependencies: {}
# [doc = " An iterator over the names of all possible captures."] # [doc = ""] # [doc = " `None` indicates an unnamed capture; the first element (capture 0, the"] # [doc = " whole matched region) is always unnamed."] # [doc = ""] # [doc = " `'r` is the lifetime of the compiled regular expression."] pub struct CaptureNames < 'r > (:: std :: slice :: Iter < 'r , Option < String > >) ;
};
}
