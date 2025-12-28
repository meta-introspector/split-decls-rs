macro_rules! MH_ALLOW_STACK_EXECUTION {
    () => {
        # [doc = " When this bit is set, all stacks in the task will be given stack execution privilege.  Only used in MH_EXECUTE filetypes."] pub const MH_ALLOW_STACK_EXECUTION : u32 = 0x20000 ;
    };
}

MH_ALLOW_STACK_EXECUTION!()