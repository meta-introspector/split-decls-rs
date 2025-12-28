macro_rules! deps {
    () => {
        BuiltinDriver!();
    };
}

macro_rules! DriverChoice {
    () => {
        deps!();
        # [doc = " The selection of the driver to use by a resource obtained with [`Platform::prepare_merge()`]."] # [doc = ""] # [doc = " If available, an index into the `drivers` field to access more diff-related information of the driver for items"] # [doc = " at the given path, as previously determined by git-attributes."] # [doc = ""] # [doc = " * `merge` is set"] # [doc = "     - Use the [`BuiltinDriver::Text`]"] # [doc = " * `-merge` is unset"] # [doc = "     - Use the [`BuiltinDriver::Binary`]"] # [doc = " * `!merge` is unspecified"] # [doc = "     - Use [`Options::default_driver`] or [`BuiltinDriver::Text`]."] # [doc = " * `merge=name`"] # [doc = "     - Search for a user-configured or built-in driver called `name`."] # [doc = "     - If not found, silently default to [`BuiltinDriver::Text`]"] # [doc = ""] # [doc = " Note that drivers are queried even if there is no object available."] # [derive (Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Debug , Hash)] pub enum DriverChoice { # [doc = " Use the given built-in driver to perform the merge."] BuiltIn (BuiltinDriver) , # [doc = " Use the user-provided driver program using the index into [the platform drivers array](Platform::drivers())."] Index (usize) , }
    };
}

DriverChoice!()