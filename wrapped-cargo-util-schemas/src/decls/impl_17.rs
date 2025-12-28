macro_rules! deps {
    () => {
        PartialVersion!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl From < semver :: Version > for PartialVersion { fn from (ver : semver :: Version) -> Self { let pre = if ver . pre . is_empty () { None } else { Some (ver . pre) } ; let build = if ver . build . is_empty () { None } else { Some (ver . build) } ; Self { major : ver . major , minor : Some (ver . minor) , patch : Some (ver . patch) , pre , build , } } }
    };
}

impl_17!();