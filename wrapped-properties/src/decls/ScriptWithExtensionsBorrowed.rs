macro_rules! deps {
    () => {
        ScriptWithExtensionsProperty!();
    };
}

macro_rules! ScriptWithExtensionsBorrowed {
    () => {
        deps!();
        # [doc = " A borrowed wrapper around script extension data, returned by"] # [doc = " [`ScriptWithExtensions::as_borrowed()`]. More efficient to query."] # [derive (Clone , Copy , Debug)] pub struct ScriptWithExtensionsBorrowed < 'a > { data : & 'a ScriptWithExtensionsProperty < 'a > , }
    };
}

ScriptWithExtensionsBorrowed!();