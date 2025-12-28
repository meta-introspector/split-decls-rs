macro_rules! deps {
    () => {
        Version!();
        Bitness!();
        Type!();
    };
}

macro_rules! Info {
    () => {
        deps!();
        # [doc = " Holds information about operating system (type, version, etc.)."] # [doc = ""] # [doc = " The best way to get string representation of the operation system information is to use its"] # [doc = " `Display` implementation."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use os_info;"] # [doc = ""] # [doc = " let info = os_info::get();"] # [doc = " println!(\"OS information: {info}\");"] # [doc = " ```"] # [derive (Debug , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Info { # [doc = " Operating system type. See `Type` for details."] pub (crate) os_type : Type , # [doc = " Operating system version. See `Version` for details."] pub (crate) version : Version , # [doc = " Operating system edition."] pub (crate) edition : Option < String > , # [doc = " Operating system codename."] pub (crate) codename : Option < String > , # [doc = " Operating system architecture in terms of how many bits compose the basic values it can deal"] # [doc = " with. See `Bitness` for details."] pub (crate) bitness : Bitness , # [doc = " Processor architecture."] pub (crate) architecture : Option < String > , }
    };
}

Info!();