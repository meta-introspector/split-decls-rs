macro_rules! XofReaderCore {
    () => {
        # [doc = " Core reader trait for extendable-output function (XOF) result."] pub trait XofReaderCore : BlockSizeUser { # [doc = " Read next XOF block."] fn read_block (& mut self) -> Block < Self > ; }
    };
}

XofReaderCore!()