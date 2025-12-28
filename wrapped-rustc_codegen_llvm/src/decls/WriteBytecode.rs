macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! WriteBytecode {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_llvm_write_bytecode)] pub (crate) struct WriteBytecode < 'a > { pub path : & 'a Path , pub err : std :: io :: Error , }
    };
}

WriteBytecode!();