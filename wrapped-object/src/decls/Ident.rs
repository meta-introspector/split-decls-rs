macro_rules! deps {
    () => {
        File!();
    };
}

macro_rules! Ident {
    () => {
        deps!();
        # [doc = " Magic number and other information."] # [doc = ""] # [doc = " Contained in the file header."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct Ident { # [doc = " Magic number. Must be `ELFMAG`."] pub magic : [u8 ; 4] , # [doc = " File class. One of the `ELFCLASS*` constants."] pub class : u8 , # [doc = " Data encoding. One of the `ELFDATA*` constants."] pub data : u8 , # [doc = " ELF version. Must be `EV_CURRENT`."] pub version : u8 , # [doc = " OS ABI identification. One of the `ELFOSABI*` constants."] pub os_abi : u8 , # [doc = " ABI version."] # [doc = ""] # [doc = " The meaning of this field depends on the `os_abi` value."] pub abi_version : u8 , # [doc = " Padding bytes."] pub padding : [u8 ; 7] , }
    };
}

Ident!()