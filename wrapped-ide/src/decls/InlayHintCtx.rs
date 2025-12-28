macro_rules! InlayHintCtx {
    () => {
        # [derive (Default)] struct InlayHintCtx { lifetime_stacks : Vec < Vec < SmolStr > > , extern_block_parent : Option < ast :: ExternBlock > , }
    };
}

InlayHintCtx!();