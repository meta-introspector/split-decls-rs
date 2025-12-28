macro_rules! CoffExportStyle {
    () => {
        # [doc = " Internal format to use for the `.drectve` section containing linker"] # [doc = " directives for symbol exports."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum CoffExportStyle { # [doc = " MSVC format supported by link.exe and LLD."] Msvc , # [doc = " Gnu format supported by GNU LD and LLD."] Gnu , }
    };
}

CoffExportStyle!()