macro_rules! deps {
    () => {
        RawAutoExtension!();
        Result!();
    };
}

macro_rules! register_auto_extension {
    () => {
        deps!();
        # [doc = " Register au auto-extension"] # [doc = ""] # [doc = " # Safety"] # [doc = " * Opening a database from an auto-extension handler will lead to"] # [doc = "   an endless recursion of the auto-handler triggering itself"] # [doc = "   indirectly for each newly-opened database."] # [doc = " * Results are undefined if the given db is closed by an auto-extension."] # [doc = " * The list of auto-extensions should not be manipulated from an auto-extension."] pub unsafe fn register_auto_extension (ax : RawAutoExtension) -> Result < () > { check (ffi :: sqlite3_auto_extension (Some (ax))) }
    };
}

register_auto_extension!();