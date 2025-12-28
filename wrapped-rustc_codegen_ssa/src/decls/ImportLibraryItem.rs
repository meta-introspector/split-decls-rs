macro_rules! ImportLibraryItem {
    () => {
        # [doc = " An item to be included in an import library."] # [doc = " This is a slimmed down version of `COFFShortExport` from `ar-archive-writer`."] pub struct ImportLibraryItem { # [doc = " The name to be exported."] pub name : String , # [doc = " The ordinal to be exported, if any."] pub ordinal : Option < u16 > , # [doc = " The original, decorated name if `name` is not decorated."] pub symbol_name : Option < String > , # [doc = " True if this is a data export, false if it is a function export."] pub is_data : bool , }
    };
}

ImportLibraryItem!();