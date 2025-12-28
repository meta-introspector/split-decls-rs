macro_rules! ListToken {
    () => {
        # [doc = " The token type for the list flavor."] # [derive (Debug)] pub (crate) struct ListToken { # [doc = " The block of slots."] block : * const u8 , # [doc = " The offset into the block."] offset : usize , }
    };
}

ListToken!();