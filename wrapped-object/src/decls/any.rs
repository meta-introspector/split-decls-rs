macro_rules! any {
    () => {
        # [cfg (any (feature = "coff" , feature = "elf" , feature = "macho" , feature = "pe" , feature = "wasm" , feature = "xcoff"))] mod any ;
    };
}

any!()