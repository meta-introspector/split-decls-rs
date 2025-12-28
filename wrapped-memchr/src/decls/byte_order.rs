macro_rules! byte_order {
    () => {
        # [test] fn byte_order () { # [cfg (target_endian = "little")] std :: eprintln ! ("LITTLE ENDIAN") ; # [cfg (target_endian = "big")] std :: eprintln ! ("BIG ENDIAN") ; }
    };
}

byte_order!();