macro_rules! Never {
    () => {
        # [doc = " An implementation of all traits that never fails, but also never finds anything."] # [derive (Debug , Copy , Clone)] pub struct Never ;
    };
}

Never!()