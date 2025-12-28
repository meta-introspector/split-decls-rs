macro_rules! FnTrait {
    () => {
        # [derive (Debug , Copy , Clone , PartialEq , Eq , Hash , PartialOrd , Ord)] pub enum FnTrait { FnOnce , FnMut , Fn , AsyncFnOnce , AsyncFnMut , AsyncFn , }
    };
}

FnTrait!();