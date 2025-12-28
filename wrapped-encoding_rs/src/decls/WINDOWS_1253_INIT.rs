macro_rules! deps {
    () => {
        Encoding!();
        VariantEncoding!();
    };
}

macro_rules! WINDOWS_1253_INIT {
    () => {
        deps!();
        # [doc = " The initializer for the [windows-1253](static.WINDOWS_1253.html) encoding."] # [doc = ""] # [doc = " For use only for taking the address of this form when"] # [doc = " Rust prohibits the use of the non-`_INIT` form directly,"] # [doc = " such as in initializers of other `static`s. If in doubt,"] # [doc = " use the corresponding non-`_INIT` reference-typed `static`."] # [doc = ""] # [doc = " This part of the public API will go away if Rust changes"] # [doc = " to make the referent of `pub const FOO: &'static Encoding`"] # [doc = " unique cross-crate or if Rust starts allowing static arrays"] # [doc = " to be initialized with `pub static FOO: &'static Encoding`"] # [doc = " items."] pub static WINDOWS_1253_INIT : Encoding = Encoding { name : "windows-1253" , variant : VariantEncoding :: SingleByte (& data :: SINGLE_BYTE_DATA . windows_1253 , 0x03A3 , 83 , 44) , } ;
    };
}

WINDOWS_1253_INIT!()