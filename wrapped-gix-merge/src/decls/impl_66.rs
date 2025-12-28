macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl < 'a > Data < 'a > { # [doc = " Return ourselves as slice of bytes if this instance stores data."] # [doc = " Note that missing data is interpreted as empty slice, to facilitate additions and deletions."] pub fn as_slice (& self) -> Option < & 'a [u8] > { match self { Data :: Buffer (d) => Some (d) , Data :: Missing => Some (& []) , Data :: TooLarge { .. } => None , } } }
    };
}

impl_66!()