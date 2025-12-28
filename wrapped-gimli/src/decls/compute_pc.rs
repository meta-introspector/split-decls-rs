macro_rules! deps {
    () => {
        Reader!();
        Error!();
        Result!();
    };
}

macro_rules! compute_pc {
    () => {
        deps!();
        fn compute_pc < R : Reader > (pc : & R , bytecode : & R , offset : i16) -> Result < R > { let pc_offset = pc . offset_from (bytecode) ; let new_pc_offset = pc_offset . wrapping_add (R :: Offset :: from_i16 (offset)) ; if new_pc_offset > bytecode . len () { Err (Error :: BadBranchTarget (new_pc_offset . into_u64 ())) } else { let mut new_pc = bytecode . clone () ; new_pc . skip (new_pc_offset) ? ; Ok (new_pc) } }
    };
}

compute_pc!();