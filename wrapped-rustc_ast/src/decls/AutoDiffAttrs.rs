macro_rules! deps {
    () => {
        DiffActivity!();
        DiffMode!();
    };
}

macro_rules! AutoDiffAttrs {
    () => {
        deps!();
        # [derive (Clone , Eq , PartialEq , Encodable , Decodable , Debug , HashStable_Generic)] pub struct AutoDiffAttrs { # [doc = " Conceptually either forward or reverse mode AD, as described in various autodiff papers and"] # [doc = " e.g. in the [JAX"] # [doc = " Documentation](https://jax.readthedocs.io/en/latest/_tutorials/advanced-autodiff.html#how-it-s-made-two-foundational-autodiff-functions)."] pub mode : DiffMode , # [doc = " A user-provided, batching width. If not given, we will default to 1 (no batching)."] # [doc = " Calling a differentiated, non-batched function through a loop 100 times is equivalent to:"] # [doc = " - Calling the function 50 times with a batch size of 2"] # [doc = " - Calling the function 25 times with a batch size of 4,"] # [doc = " etc. A batched function takes more (or longer) arguments, and might be able to benefit from"] # [doc = " cache locality, better re-usal of primal values, and other optimizations."] # [doc = " We will (before LLVM's vectorizer runs) just generate most LLVM-IR instructions `width`"] # [doc = " times, so this massively increases code size. As such, values like 1024 are unlikely to"] # [doc = " work. We should consider limiting this to u8 or u16, but will leave it at u32 for"] # [doc = " experiments for now and focus on documenting the implications of a large width."] pub width : u32 , pub ret_activity : DiffActivity , pub input_activity : Vec < DiffActivity > , }
    };
}

AutoDiffAttrs!()