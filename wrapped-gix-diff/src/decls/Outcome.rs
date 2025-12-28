macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! Outcome {
    () => {
        deps!();
        # [doc = " The outcome returned by [Pipeline::convert_to_diffable()](super::Pipeline::convert_to_diffable())."] # [derive (Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Debug)] pub struct Outcome { # [doc = " If available, an index into the `drivers` field to access more diff-related information of the driver for items"] # [doc = " at the given path, as previously determined by git-attributes."] # [doc = ""] # [doc = " Note that drivers are queried even if there is no object available."] pub driver_index : Option < usize > , # [doc = " The data itself, suitable for diffing, and if the object or worktree item is present at all."] pub data : Option < Data > , }
    };
}

Outcome!()