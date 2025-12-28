macro_rules! deps {
    () => {
        Relocation!();
    };
}

macro_rules! R_386_TLS_LDM_CALL {
    () => {
        deps!();
        # [doc = " Relocation for call to __tls_get_addr() in LDM code"] pub const R_386_TLS_LDM_CALL : u32 = 30 ;
    };
}

R_386_TLS_LDM_CALL!();