macro_rules! AIX_METADATA_SYMBOL_NAME {
    () => {
        static AIX_METADATA_SYMBOL_NAME : & 'static str = "__aix_rust_metadata" ;
    };
}

AIX_METADATA_SYMBOL_NAME!()