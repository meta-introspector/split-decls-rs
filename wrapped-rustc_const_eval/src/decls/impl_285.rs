macro_rules! deps {
    () => {
        MemPlaceMeta!();
    };
}

macro_rules! impl_285 {
    () => {
        deps!();
        impl < Prov : Provenance > MemPlaceMeta < Prov > { # [cfg_attr (debug_assertions , track_caller)] pub fn unwrap_meta (self) -> Scalar < Prov > { match self { Self :: Meta (s) => s , Self :: None => { bug ! ("expected wide pointer extra data (e.g. slice length or trait object vtable)") } } } # [inline (always)] pub fn has_meta (self) -> bool { match self { Self :: Meta (_) => true , Self :: None => false , } } }
    };
}

impl_285!();