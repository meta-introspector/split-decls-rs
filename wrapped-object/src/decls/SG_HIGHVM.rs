macro_rules! SG_HIGHVM {
    () => {
        # [doc = " the file contents for this segment is for the high part of the VM space, the low part is zero filled (for stacks in core files)"] pub const SG_HIGHVM : u32 = 0x1 ;
    };
}

SG_HIGHVM!()