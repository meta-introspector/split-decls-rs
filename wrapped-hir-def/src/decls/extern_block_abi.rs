macro_rules! deps {
    () => {
        DefDatabase!();
    };
}

macro_rules! extern_block_abi {
    () => {
        deps!();
        pub (crate) fn extern_block_abi (db : & dyn DefDatabase , extern_block : ExternBlockId ,) -> Option < Symbol > { let source = extern_block . lookup (db) . source (db) ; source . value . abi () . map (| abi | { match abi . abi_string () { Some (tok) => Symbol :: intern (tok . text_without_quotes ()) , _ => sym :: C , } }) }
    };
}

extern_block_abi!();