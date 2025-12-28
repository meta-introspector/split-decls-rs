macro_rules! deps {
    () => {
        Relocation!();
    };
}

macro_rules! R_386_TLS_GD_CALL {
    () => {
        deps!();
        # [doc = " Relocation for call to __tls_get_addr()"] pub const R_386_TLS_GD_CALL : u32 = 26 ;
    };
}

R_386_TLS_GD_CALL!()