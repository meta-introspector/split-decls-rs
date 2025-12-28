macro_rules! CallUnstable {
    () => {
        # [doc = " A call to an `#[unstable]` const fn, `#[rustc_const_unstable]` function or trait."] # [doc = ""] # [doc = " Contains the name of the feature that would allow the use of this function/trait."] # [derive (Debug)] pub (crate) struct CallUnstable { pub def_id : DefId , pub feature : Symbol , # [doc = " If this is true, then the feature is enabled, but we need to still check if it is safe to"] # [doc = " expose on stable."] pub feature_enabled : bool , pub safe_to_expose_on_stable : bool , # [doc = " true if `def_id` is the function we are calling, false if `def_id` is an unstable trait."] pub is_function_call : bool , }
    };
}

CallUnstable!();