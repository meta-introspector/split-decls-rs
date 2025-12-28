macro_rules! ParseContextState {
    () => {
        # [derive (Debug , Default , Clone , Copy)] struct ParseContextState { recursion_level : u32 , in_conversion : bool , }
    };
}

ParseContextState!();