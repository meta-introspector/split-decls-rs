macro_rules! TrySelectError {
    () => {
        # [doc = " An error returned from the [`try_select`] method."] # [doc = ""] # [doc = " Failed because none of the channel operations were ready."] # [doc = ""] # [doc = " [`try_select`]: super::Select::try_select"] # [derive (PartialEq , Eq , Clone , Copy , Debug)] pub struct TrySelectError ;
    };
}

TrySelectError!()