macro_rules! gnu_compression {
    () => {
        # [cfg (any (feature = "elf" , feature = "macho"))] mod gnu_compression ;
    };
}

gnu_compression!()