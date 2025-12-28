macro_rules! Params {
    () => {
        # [derive (Default , Clone , PartialEq , Eq)] pub struct Params { # [doc = " Number of subparameters for each parameter."] # [doc = ""] # [doc = " For each entry in the `params` slice, this stores the length of the param as number of"] # [doc = " subparams at the same index as the param in the `params` slice."] # [doc = ""] # [doc = " At the subparam positions the length will always be `0`."] subparams : [u8 ; MAX_PARAMS] , # [doc = " All parameters and subparameters."] params : [u16 ; MAX_PARAMS] , # [doc = " Number of suparameters in the current parameter."] current_subparams : u8 , # [doc = " Total number of parameters and subparameters."] len : usize , }
    };
}

Params!()