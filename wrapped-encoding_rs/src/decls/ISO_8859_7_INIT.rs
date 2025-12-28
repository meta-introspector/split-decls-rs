macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! ISO_8859_7_INIT {
    () => {
        deps!();
        # [doc = " The initializer for the [ISO-8859-7](static.ISO_8859_7.html) encoding."] # [doc = ""] # [doc = " For use only for taking the address of this form when"] # [doc = " Rust prohibits the use of the non-`_INIT` form directly,"] # [doc = " such as in initializers of other `static`s. If in doubt,"] # [doc = " use the corresponding non-`_INIT` reference-typed `static`."] # [doc = ""] # [doc = " This part of the public API will go away if Rust changes"] # [doc = " to make the referent of `pub const FOO: &'static Encoding`"] # [doc = " unique cross-crate or if Rust starts allowing static arrays"] # [doc = " to be initialized with `pub static FOO: &'static Encoding`"] # [doc = " items."] pub static ISO_8859_7_INIT : Encoding = Encoding { name : "ISO-8859-7" , variant : VariantEncoding :: SingleByte (& data :: SINGLE_BYTE_DATA . iso_8859_7 , 0x03A3 , 83 , 44) , } ;
    };
}

ISO_8859_7_INIT!()