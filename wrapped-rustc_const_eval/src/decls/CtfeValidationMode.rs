macro_rules! CtfeValidationMode {
    () => {
        # [doc = " Extra things to check for during validation of CTFE results."] # [derive (Copy , Clone)] pub enum CtfeValidationMode { # [doc = " Validation of a `static`"] Static { mutbl : Mutability } , # [doc = " Validation of a promoted."] Promoted , # [doc = " Validation of a `const`."] # [doc = " `allow_immutable_unsafe_cell` says whether we allow `UnsafeCell` in immutable memory (which is the"] # [doc = " case for the top-level allocation of a `const`, where this is fine because the allocation will be"] # [doc = " copied at each use site)."] Const { allow_immutable_unsafe_cell : bool } , }
    };
}

CtfeValidationMode!()