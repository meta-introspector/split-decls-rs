macro_rules! deps {
    () => {
        PartialVersion!();
        Result!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl Display for PartialVersion { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let major = self . major ; write ! (f , "{major}") ? ; if let Some (minor) = self . minor { write ! (f , ".{minor}") ? ; } if let Some (patch) = self . patch { write ! (f , ".{patch}") ? ; } if let Some (pre) = self . pre . as_ref () { write ! (f , "-{pre}") ? ; } if let Some (build) = self . build . as_ref () { write ! (f , "+{build}") ? ; } Ok (()) } }
    };
}

impl_19!()