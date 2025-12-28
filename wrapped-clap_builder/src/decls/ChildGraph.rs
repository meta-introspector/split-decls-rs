macro_rules! deps {
    () => {
        Child!();
    };
}

macro_rules! ChildGraph {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct ChildGraph < T > (Vec < Child < T > >) ;
    };
}

ChildGraph!()