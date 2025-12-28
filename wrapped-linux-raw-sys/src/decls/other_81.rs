macro_rules! other_81 {
    () => {
        # [cfg (target_pointer_width = "64")] # [repr (C)] # [derive (Copy , Clone)] pub union Elf_Dyn_Union { pub d_val : u64 , pub d_ptr : usize , }
    };
}

other_81!()