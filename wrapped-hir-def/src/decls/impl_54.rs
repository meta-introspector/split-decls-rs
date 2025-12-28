macro_rules! deps {
    () => {
        ImportId!();
        ImportOrGlob!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl ImportOrGlob { pub fn into_import (self) -> Option < ImportId > { match self { ImportOrGlob :: Import (it) => Some (it) , _ => None , } } }
    };
}

impl_54!()