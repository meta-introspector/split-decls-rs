macro_rules! Status {
    () => {
        # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Status { Unstable { # [doc = " The feature that must be enabled to use this operation."] gate : Symbol , # [doc = " Whether the feature gate was already checked (because the logic is a bit more"] # [doc = " complicated than just checking a single gate)."] gate_already_checked : bool , # [doc = " Whether it is allowed to use this operation from stable `const fn`."] # [doc = " This will usually be `false`."] safe_to_expose_on_stable : bool , # [doc = " We indicate whether this is a function call, since we can use targeted"] # [doc = " diagnostics for \"callee is not safe to expose om stable\"."] is_function_call : bool , } , Forbidden , }
    };
}

Status!();