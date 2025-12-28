macro_rules! deps {
    () => {
        Allocator!();
    };
}

macro_rules! Global {
    () => {
        deps!();
        # [doc = " The global memory allocator."] # [doc = ""] # [doc = " This type implements the [`Allocator`] trait by forwarding calls"] # [doc = " to the allocator registered with the `#[global_allocator]` attribute"] # [doc = " if there is one, or the `std` crate’s default."] # [doc = ""] # [doc = " Note: while this type is unstable, the functionality it provides can be"] # [doc = " accessed through the [free functions in `alloc`](crate#functions)."] # [derive (Copy , Clone , Default , Debug)] pub struct Global ;
    };
}

Global!();