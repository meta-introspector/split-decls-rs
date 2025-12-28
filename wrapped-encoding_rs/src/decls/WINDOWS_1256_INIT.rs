macro_rules! deps {
    () => {
        Encoding!();
        VariantEncoding!();
    };
}

macro_rules! WINDOWS_1256_INIT {
    () => {
        deps!();
        # [doc = " The initializer for the [windows-1256](static.WINDOWS_1256.html) encoding."] # [doc = ""] # [doc = " For use only for taking the address of this form when"] # [doc = " Rust prohibits the use of the non-`_INIT` form directly,"] # [doc = " such as in initializers of other `static`s. If in doubt,"] # [doc = " use the corresponding non-`_INIT` reference-typed `static`."] # [doc = ""] # [doc = " This part of the public API will go away if Rust changes"] # [doc = " to make the referent of `pub const FOO: &'static Encoding`"] # [doc = " unique cross-crate or if Rust starts allowing static arrays"] # [doc = " to be initialized with `pub static FOO: &'static Encoding`"] # [doc = " items."] pub static WINDOWS_1256_INIT : Encoding = Encoding { name : "windows-1256" , variant : VariantEncoding :: SingleByte (& data :: SINGLE_BYTE_DATA . windows_1256 , 0x0621 , 65 , 22) , } ;
    };
}

WINDOWS_1256_INIT!();