macro_rules! deps {
    () => {
        BuilderPush!();
    };
}

macro_rules! impl_346 {
    () => {
        deps!();
        # [cfg (debug_assertions)] impl Drop for BuilderPush { fn drop (& mut self) { panic ! ("Found a `push` without a `pop`.") ; } }
    };
}

impl_346!()