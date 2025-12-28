macro_rules! InlinedFunction {
    () => {
        pub (crate) struct InlinedFunction < R : gimli :: Reader > { pub (crate) dw_die_offset : gimli :: UnitOffset < R :: Offset > , pub (crate) name : Option < R > , pub (crate) call_file : Option < u64 > , pub (crate) call_line : u32 , pub (crate) call_column : u32 , }
    };
}

InlinedFunction!()