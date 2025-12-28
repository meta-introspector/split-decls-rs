macro_rules! ExportData {
    () => {
        # [doc = " Terminal data for an exports trie node."] # [derive (Debug)] pub enum ExportData < 'data > { # [doc = " A regular export."] Regular { # [doc = " The address of the export."] address : u64 , } , # [doc = " A re-exported symbol."] Reexport { # [doc = " The ordinal of the dylib to re-export from."] dylib_ordinal : u64 , # [doc = " The name of the symbol to re-export."] import_name : & 'data [u8] , } , # [doc = " A stub-and-resolver symbol."] StubAndResolver { # [doc = " The address of the stub."] stub_address : u64 , # [doc = " The address of the resolver."] resolver_address : u64 , } , }
    };
}

ExportData!()