macro_rules! deps {
    () => {
        Encoding!();
        VariantEncoding!();
    };
}

macro_rules! REPLACEMENT_INIT {
    () => {
        deps!();
        # [doc = " The initializer for the [replacement](static.REPLACEMENT.html) encoding."] # [doc = ""] # [doc = " For use only for taking the address of this form when"] # [doc = " Rust prohibits the use of the non-`_INIT` form directly,"] # [doc = " such as in initializers of other `static`s. If in doubt,"] # [doc = " use the corresponding non-`_INIT` reference-typed `static`."] # [doc = ""] # [doc = " This part of the public API will go away if Rust changes"] # [doc = " to make the referent of `pub const FOO: &'static Encoding`"] # [doc = " unique cross-crate or if Rust starts allowing static arrays"] # [doc = " to be initialized with `pub static FOO: &'static Encoding`"] # [doc = " items."] pub static REPLACEMENT_INIT : Encoding = Encoding { name : "replacement" , variant : VariantEncoding :: Replacement , } ;
    };
}

REPLACEMENT_INIT!();