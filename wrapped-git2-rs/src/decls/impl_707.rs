macro_rules! deps {
    () => {
        Object!();
        Revspec!();
    };
}

macro_rules! impl_707 {
    () => {
        deps!();
        impl < 'repo > Revspec < 'repo > { # [doc = " Assembles a new revspec from the from/to components."] pub fn from_objects (from : Option < Object < 'repo > > , to : Option < Object < 'repo > > , mode : RevparseMode ,) -> Revspec < 'repo > { Revspec { from , to , mode } } # [doc = " Access the `from` range of this revspec."] pub fn from (& self) -> Option < & Object < 'repo > > { self . from . as_ref () } # [doc = " Access the `to` range of this revspec."] pub fn to (& self) -> Option < & Object < 'repo > > { self . to . as_ref () } # [doc = " Returns the intent of the revspec."] pub fn mode (& self) -> RevparseMode { self . mode } }
    };
}

impl_707!();