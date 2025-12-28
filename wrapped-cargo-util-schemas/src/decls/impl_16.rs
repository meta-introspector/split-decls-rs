macro_rules! deps {
    () => {
        PartialVersion!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl PartialVersion { pub fn to_version (& self) -> Option < Version > { Some (Version { major : self . major , minor : self . minor ? , patch : self . patch ? , pre : self . pre . clone () . unwrap_or_default () , build : self . build . clone () . unwrap_or_default () , }) } pub fn to_caret_req (& self) -> VersionReq { VersionReq { comparators : vec ! [Comparator { op : semver :: Op :: Caret , major : self . major , minor : self . minor , patch : self . patch , pre : self . pre . as_ref () . cloned () . unwrap_or_default () , }] , } } # [doc = " Check if this matches a version, including build metadata"] # [doc = ""] # [doc = " Build metadata does not affect version precedence but may be necessary for uniquely"] # [doc = " identifying a package."] pub fn matches (& self , version : & Version) -> bool { if ! version . pre . is_empty () && self . pre . is_none () { return false ; } self . major == version . major && self . minor . map (| f | f == version . minor) . unwrap_or (true) && self . patch . map (| f | f == version . patch) . unwrap_or (true) && self . pre . as_ref () . map (| f | f == & version . pre) . unwrap_or (true) && self . build . as_ref () . map (| f | f == & version . build) . unwrap_or (true) } }
    };
}

impl_16!()