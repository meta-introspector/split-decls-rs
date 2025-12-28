macro_rules! Header {
    () => {
        # [doc = " A parsed section header, containing a name and optionally a subsection name."] # [derive (Clone , Eq , PartialEq , Ord , PartialOrd , Hash , Debug)] pub struct Header < 'a > { # [doc = " The name of the header."] pub (crate) name : Name < 'a > , # [doc = " The separator used to determine if the section contains a subsection."] # [doc = " This is either a period `.` or a string of whitespace. Note that"] # [doc = " reconstruction of subsection format is dependent on this value. If this"] # [doc = " is all whitespace, then the subsection name needs to be surrounded by"] # [doc = " quotes to have perfect reconstruction."] pub (crate) separator : Option < Cow < 'a , BStr > > , pub (crate) subsection_name : Option < Cow < 'a , BStr > > , }
    };
}

Header!()