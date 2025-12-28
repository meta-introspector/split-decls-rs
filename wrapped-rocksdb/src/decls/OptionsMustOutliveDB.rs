macro_rules! deps {
    () => {
        Env!();
        WriteBufferManager!();
        BlockBasedOptionsMustOutliveDB!();
        Cache!();
    };
}

macro_rules! OptionsMustOutliveDB {
    () => {
        deps!();
        # [derive (Default)] pub (crate) struct OptionsMustOutliveDB { env : Option < Env > , row_cache : Option < Cache > , blob_cache : Option < Cache > , block_based : Option < BlockBasedOptionsMustOutliveDB > , write_buffer_manager : Option < WriteBufferManager > , }
    };
}

OptionsMustOutliveDB!();