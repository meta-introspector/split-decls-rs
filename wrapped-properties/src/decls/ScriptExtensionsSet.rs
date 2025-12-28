macro_rules! deps {
    () => {
        Script!();
    };
}

macro_rules! ScriptExtensionsSet {
    () => {
        deps!();
        # [doc = " A struct that wraps a [`Script`] array, such as in the return value for"] # [doc = " [`get_script_extensions_val()`](ScriptWithExtensionsBorrowed::get_script_extensions_val)."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub struct ScriptExtensionsSet < 'a > { values : & 'a ZeroSlice < Script > , }
    };
}

ScriptExtensionsSet!();