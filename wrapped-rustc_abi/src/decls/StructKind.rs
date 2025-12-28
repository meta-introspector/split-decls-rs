macro_rules! deps {
    () => {
        Align!();
        Size!();
    };
}

macro_rules! StructKind {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug)] pub enum StructKind { # [doc = " A tuple, closure, or univariant which cannot be coerced to unsized."] AlwaysSized , # [doc = " A univariant, the last field of which may be coerced to unsized."] MaybeUnsized , # [doc = " A univariant, but with a prefix of an arbitrary size & alignment (e.g., enum tag)."] Prefixed (Size , Align) , }
    };
}

StructKind!()