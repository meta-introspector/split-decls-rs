macro_rules! deps {
    () => {
        GlobId!();
        ImportOrExternCrate!();
        ImportOrGlob!();
        ImportId!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl ImportOrExternCrate { pub fn import_or_glob (self) -> Option < ImportOrGlob > { match self { ImportOrExternCrate :: Import (it) => Some (ImportOrGlob :: Import (it)) , ImportOrExternCrate :: Glob (it) => Some (ImportOrGlob :: Glob (it)) , _ => None , } } pub fn import (self) -> Option < ImportId > { match self { ImportOrExternCrate :: Import (it) => Some (it) , _ => None , } } pub fn glob (self) -> Option < GlobId > { match self { ImportOrExternCrate :: Glob (id) => Some (id) , _ => None , } } pub fn use_ (self) -> Option < UseId > { match self { ImportOrExternCrate :: Glob (id) => Some (id . use_) , ImportOrExternCrate :: Import (id) => Some (id . use_) , _ => None , } } }
    };
}

impl_52!();