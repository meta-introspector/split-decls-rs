macro_rules! RawEnum {
    () => {
        # [doc = " Wrapper for a raw enum value returned from LLVM's C APIs."] # [doc = ""] # [doc = " For C enums returned by LLVM, it's risky to use a Rust enum as the return"] # [doc = " type, because it would be UB if a later version of LLVM adds a new enum"] # [doc = " value and returns it. Instead, return this raw wrapper, then convert to the"] # [doc = " Rust-side enum explicitly."] # [repr (transparent)] pub (crate) struct RawEnum < T > { value : u32 , # [doc = " We don't own or consume a `T`, but we can produce one."] _rust_side_type : PhantomData < fn () -> T > , }
    };
}

RawEnum!();