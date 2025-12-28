macro_rules! deps {
    () => {
        ProgramKind!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl < 'a > From < & 'a OsStr > for ProgramKind { fn from (v : & 'a OsStr) -> Self { let p = std :: path :: Path :: new (v) ; match p . file_stem () . and_then (OsStr :: to_str) { None => ProgramKind :: Simple , Some (stem) => { if stem . eq_ignore_ascii_case ("ssh") { ProgramKind :: Ssh } else if stem . eq_ignore_ascii_case ("plink") { ProgramKind :: Plink } else if stem . eq_ignore_ascii_case ("putty") { ProgramKind :: Putty } else if stem . eq_ignore_ascii_case ("tortoiseplink") { ProgramKind :: TortoisePlink } else { ProgramKind :: Simple } } } } }
    };
}

impl_129!();