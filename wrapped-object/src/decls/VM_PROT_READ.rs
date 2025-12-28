macro_rules! VM_PROT_READ {
    () => {
        # [doc = " read permission"] pub const VM_PROT_READ : u32 = 0x01 ;
    };
}

VM_PROT_READ!();