macro_rules! deps {
    () => {
        StdError!();
    };
}

macro_rules! ChainState {
    () => {
        deps!();
        # [derive (Clone)] pub (crate) enum ChainState < 'a > { Linked { next : Option < & 'a (dyn StdError + 'static) > , } , # [cfg (any (feature = "std" , not (anyhow_no_core_error)))] Buffered { rest : vec :: IntoIter < & 'a (dyn StdError + 'static) > , } , }
    };
}

ChainState!();