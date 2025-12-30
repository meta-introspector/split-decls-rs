// Generated macro for Multi (struct)
macro_rules! Depcrate_multiMulti {
() => {
// Module: crate::multi
// Provides: {"Multi"}
// Dependencies: {}
# [doc = " A multi handle for initiating multiple connections simultaneously."] # [doc = ""] # [doc = " This structure corresponds to `CURLM` in libcurl and provides the ability to"] # [doc = " have multiple transfers in flight simultaneously. This handle is then used"] # [doc = " to manage each transfer. The main purpose of a `CURLM` is for the"] # [doc = " *application* to drive the I/O rather than libcurl itself doing all the"] # [doc = " blocking. Methods like `action` allow the application to inform libcurl of"] # [doc = " when events have happened."] # [doc = ""] # [doc = " Lots more documentation can be found on the libcurl [multi tutorial] where"] # [doc = " the APIs correspond pretty closely with this crate."] # [doc = ""] # [doc = " [multi tutorial]: https://curl.haxx.se/libcurl/c/libcurl-multi.html"] pub struct Multi { raw : Arc < RawMulti > , data : Box < MultiData > , }
};
}
