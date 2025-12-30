// Generated macro for GB18030_INIT (static)
macro_rules! DepcrateGB18030_INIT {
() => {
// Module: crate
// Provides: {"GB18030_INIT"}
// Dependencies: {}
# [doc = " The initializer for the [gb18030](static.GB18030.html) encoding."] # [doc = ""] # [doc = " For use only for taking the address of this form when"] # [doc = " Rust prohibits the use of the non-`_INIT` form directly,"] # [doc = " such as in initializers of other `static`s. If in doubt,"] # [doc = " use the corresponding non-`_INIT` reference-typed `static`."] # [doc = ""] # [doc = " This part of the public API will go away if Rust changes"] # [doc = " to make the referent of `pub const FOO: &'static Encoding`"] # [doc = " unique cross-crate or if Rust starts allowing static arrays"] # [doc = " to be initialized with `pub static FOO: &'static Encoding`"] # [doc = " items."] pub static GB18030_INIT : Encoding = Encoding { name : "gb18030" , variant : VariantEncoding :: Gb18030 , } ;
};
}
