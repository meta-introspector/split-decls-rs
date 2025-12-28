macro_rules! FileEntryFormat {
    () => {
        # [doc = " The format of a component of an include directory or file name entry."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub struct FileEntryFormat { # [doc = " The type of information that is represented by the component."] pub content_type : constants :: DwLnct , # [doc = " The encoding form of the component value."] pub form : constants :: DwForm , }
    };
}

FileEntryFormat!();