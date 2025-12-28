macro_rules! deps {
    () => {
        Global!();
        Unique!();
        Allocator!();
    };
}

macro_rules! Box {
    () => {
        deps!();
        # [doc = " A pointer type for heap allocation."] # [doc = ""] # [doc = " See the [module-level documentation](../../std/boxed/index.html) for more."] pub struct Box < T : ? Sized , A : Allocator = Global > (Unique < T > , A) ;
    };
}

Box!()