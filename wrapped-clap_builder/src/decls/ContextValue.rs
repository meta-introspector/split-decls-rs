macro_rules! deps {
    () => {
        StyledStr!();
        ContextKind!();
    };
}

macro_rules! ContextValue {
    () => {
        deps!();
        # [doc = " A piece of error information"] # [derive (Clone , Debug , PartialEq , Eq)] # [non_exhaustive] # [cfg (feature = "error-context")] pub enum ContextValue { # [doc = " [`ContextKind`] is self-sufficient, no additional information needed"] None , # [doc = " A single value"] Bool (bool) , # [doc = " A single value"] String (String) , # [doc = " Many values"] Strings (Vec < String >) , # [doc = " A single value"] StyledStr (crate :: builder :: StyledStr) , # [doc = " many value"] StyledStrs (Vec < crate :: builder :: StyledStr >) , # [doc = " A single value"] Number (isize) , }
    };
}

ContextValue!();